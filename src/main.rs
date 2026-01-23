// src/main.rs

use anyhow::{Result, bail};

use std::env;
use std::path::PathBuf;
use clap::Parser;

mod core;

use crate::core::helpers::init_logger;
use crate::core::helpers::get_data_dir;
use crate::core::helpers::get_final_backup_path;
use crate::core::helpers::parse_decimal;
use crate::core::helpers::parse_optional_decimal;

use crate::core::command::{
    Cli,
    LedgerCommand,
    DataCommand,
    SystemCommand,
    ReportCommand,
    MaintenanceCommand,
    ExportImportFormat,
};
use crate::core::wallet::{
    Codexi,
    OperationKind,
    OperationFlow,
    RegularKind,
};


fn main() {
    let cli = Cli::parse();

    let is_maintenance = matches!(cli.command, LedgerCommand::Maintenance(_));

    if let Err(e) = app(cli) {
        eprintln!("{e}");

        // Message seulement si ce n'est PAS une commande maintenance
        if !is_maintenance {
            eprintln!("Please try: `codexi maintenance migrate 2`");
        }

        std::process::exit(1);
    }
}

pub fn app(cli: Cli) -> Result<()> {
    init_logger(false);
    let data_dir = get_data_dir()?;

    match cli.command {

        // Commande spécial : No current load
        LedgerCommand::Maintenance(maint_args) => {
            match maint_args.command {
                MaintenanceCommand::Migrate{ version } => {
                    match version {
                        2 => {
                            let archives = Codexi::list_archives()?;
                            let _ = Codexi::migrate_v1(&data_dir, &archives)?;
                            log::info!("Migrate done");
                        },
                        _ => bail!("Migration not supported")
                    }
                },
                MaintenanceCommand::Clear => {
                    let snapshots = Codexi::list_snapshot()?;
                    let archives = Codexi::list_archives()?;
                    Codexi::clear(&snapshots, &archives)?;
                    log::warn!("Codexi cleared (all data files deleted include snapshots and archives)");
                },
                MaintenanceCommand::LedgerInfos => {
                    let codexi = Codexi::load_current_ledger(&data_dir)?;
                    let archives = Codexi::list_archives()?;
                    let infos = codexi.ledger_infos(&archives)?;
                    Codexi::view_ledger_infos(&infos);
                },
            }
        }
        // Others commands for current ledger
        cmd => {
            let mut codexi = match Codexi::load_current_ledger(&data_dir) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(2);
                }
            };
            handle_ledger_command(cmd, &mut codexi, &data_dir)?;
        },
    }

    Ok(())
}

fn handle_ledger_command(
    command: LedgerCommand,
    codexi: &mut Codexi,
    data_dir: &PathBuf,
) -> Result<()> {

    // current directory
    let cwd = env::current_dir()?;

    match command {

        LedgerCommand::Debit { date, amount, description } => {
            let amount_d = parse_decimal(&amount, "amount")?;
            codexi.add_operation(
                OperationKind::Regular(RegularKind::Transaction),
                OperationFlow::Debit,
                &date,
                amount_d,
                &description.join(" "),
                None,
            )?;
            codexi.save_current_ledger(&data_dir)?;
        },

        LedgerCommand::Credit { date, amount, description } => {
            let amount_d = parse_decimal(&amount, "amount")?;
            codexi.add_operation(
                OperationKind::Regular(RegularKind::Transaction),
                OperationFlow::Credit,
                &date,
                amount_d,
                &description.join(" "),
                None,
            )?;
            codexi.save_current_ledger(&data_dir)?;
        },

        LedgerCommand::Search { from, to, text, kind, flow, amount_min, amount_max, latest } => {
            let amount_min_d = parse_optional_decimal(&amount_min, "amount_min")?;
            let amount_max_d = parse_optional_decimal(&amount_max, "amount_max")?;
            let search_items = codexi.search(from, to, text, kind, flow, amount_min_d, amount_max_d, latest)?;
            if search_items.is_empty() {
                log::warn!("No data available as per criteria.");
            } else {
                Codexi::view_search(&search_items);
            }
        },

        LedgerCommand::Data(data_args) => {
            match data_args.command {
                DataCommand::Export { format } => {
                    match format {
                        ExportImportFormat::Csv => codexi.export_csv(&cwd)?,
                        ExportImportFormat::Toml => codexi.export_toml(&cwd)?,
                        ExportImportFormat::Json => codexi.export_json(&cwd)?,
                    }
                }
                DataCommand::Import { format } => {
                    match format {
                        ExportImportFormat::Csv => {
                            let codexi = Codexi::import_csv(&cwd)?;
                            codexi.save_current_ledger(&data_dir)?;
                        },
                        ExportImportFormat::Toml => {
                            let codexi = Codexi::import_toml(&cwd)?;
                            codexi.save_current_ledger(&data_dir)?;
                        },
                        ExportImportFormat::Json => {
                            let codexi = Codexi::import_json(&cwd)?;
                            codexi.save_current_ledger(&data_dir)?;
                        },
                    }
                }

                DataCommand::Restore{ snapshot_file } => {
                    let codexi = Codexi::restore_snapshot(&snapshot_file)?;
                    codexi.save_current_ledger(&data_dir)?;
                }

                DataCommand::List{} => {
                    let snapshot = Codexi::list_snapshot()?;
                    Codexi::view_snapshot(&snapshot);
                }
                // create a snapshot
                DataCommand::Snapshot{} => {
                    let _ = codexi.snapshot()?;
                }

                DataCommand::Clean{ keep } => {
                    let snapshots = Codexi::list_snapshot()?;
                    Codexi::clean_snapshot(&snapshots, keep)?;
                }
            }
        },

        LedgerCommand::System(system_args) => {
            match system_args.command {
                SystemCommand::Init { date, initial_amount } => {
                    let initial_amount_d = parse_decimal(&initial_amount, "initial_amount")?;
                    codexi.initialize(initial_amount_d, &date)?;
                    codexi.save_current_ledger(&data_dir)?;
                },
                SystemCommand::Adjust { date, physical_amount} => {
                    let physical_amount_d = parse_decimal(&physical_amount, "physical_amount")?;
                    codexi.adjust_balance(physical_amount_d, &date)?;
                    codexi.save_current_ledger(&data_dir)?;
                },
                SystemCommand::Close { date, description } => {
                    codexi.close_period(&date, description)?;
                    codexi.save_current_ledger(&data_dir)?;
                },
                SystemCommand::Void { index } => {
                    codexi.void_operation(index)?;
                    codexi.save_current_ledger(&data_dir)?;
                },
                SystemCommand::List {} => {
                    let results = Codexi::list_archives()?;
                    Codexi::view_archive(&results);
                },
                SystemCommand::View {filename} => {
                    let codexi = Codexi::load_archive(&filename)?;
                    let results = codexi.search(None, None, None, None, None, None, None, None)?;
                    Codexi::view_search(&results);
                },
                SystemCommand::Backup{ target_dir } => {
                    let final_backup_path = get_final_backup_path(target_dir.as_deref())?;
                    Codexi::backup(&final_backup_path)?;
                },
                SystemCommand::Restore{ filename } => {
                    let full_path = PathBuf::from(filename);
                    Codexi::restore(&full_path)?;
                },
            }
        },

        LedgerCommand::Report(report_args) => {
            match report_args.command {
                ReportCommand::Balance { from, to } => {
                    let balance_items = codexi.search(from, to, None, None, None, None, None, None)?;
                    if let Some(results) = codexi.balance(&balance_items) {
                        Codexi::view_balance(&results);
                    }
                },
                ReportCommand::Stats { from, to, net } => {
                    let stats_items = codexi.search(from, to, None, None, None, None, None, None)?;
                    if let Some(stats) = codexi.stats(&stats_items, net) {
                        Codexi::view_stats(&stats);
                    }
                },
                ReportCommand::Resume {} => {
                    let resume_items = codexi.search(None, None, None, None, None, None, None, None)?;
                    if let Some(resume) = codexi.resume(&resume_items) {
                        Codexi::view_resume(&resume);
                    }
                },
            }
        },
        LedgerCommand::Maintenance(_maint_args) => {},
    }
    Ok(())
}
