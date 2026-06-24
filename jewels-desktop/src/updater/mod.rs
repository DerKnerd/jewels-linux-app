use futures_util::StreamExt;
use libjewels::dbus::aur::AurProxy;
use libjewels::dbus::get_bus;
use libjewels::dbus::pacman::PacmanProxy;
use libjewels::dbus::screensaver::ScreenSaverProxy;
use progress_bar::{
    Color, ProgressStyle, Style, finalize_progress_bar, inc_progress_bar, init_progress_bar,
    print_progress_bar_info, set_action_width, set_progress_bar_action, set_progress_bar_max,
    set_progress_bar_progress, set_progress_bar_width, set_progress_style,
};
use tokio::select;
use zbus::Connection;

pub async fn update_system() -> anyhow::Result<()> {
    let session_bus = Connection::session().await?;
    let screen_saver_proxy = ScreenSaverProxy::new(&session_bus).await?;
    let cookie = screen_saver_proxy
        .inhibit("Jewels", "Updating system")
        .await?;
    let res = {
        let conn = get_bus().await?;
        let pacman = PacmanProxy::new(&conn).await?;
        let mut download = pacman.receive_download().await?;
        let mut update = pacman.receive_update().await?;
        let mut failure = pacman.receive_failure().await?;
        let mut finished = pacman.receive_finished().await?;

        let aur_proxy = AurProxy::new(&conn).await?;
        let upgrade_aur_packages =
            aur_proxy.get_available_updates().await.unwrap_or_default();
        let aur_packages_count = upgrade_aur_packages.len();

        pacman.install_updates(1).await?;
        let update_aur = {
            move || async move {
                let mut update = aur_proxy.receive_update().await?;
                let mut failure = aur_proxy.receive_failure().await?;
                let mut finished = aur_proxy.receive_finished().await?;
                let mut build_started = aur_proxy.receive_build_started().await?;
                let mut built = aur_proxy.receive_built().await?;
                let mut failed = aur_proxy.receive_failed().await?;

                aur_proxy.install_updates().await?;

                set_progress_bar_progress(0);
                set_progress_bar_max(aur_packages_count);
                set_progress_bar_action("Updating AUR", Color::Blue, Style::Bold);

                loop {
                    select! {
                        Some(signal) = update.next() => {
                            if let Ok(args) = signal.args() {
                                if args.progress.current == 0 {
                                    set_progress_bar_max(args.progress.howmany);
                                    set_progress_bar_progress(0);
                                }
                                set_progress_bar_action(args.progress.package.as_str(), Color::Blue, Style::Bold);
                                set_progress_bar_progress(args.progress.current);
                            }
                        },
                        Some(signal) = build_started.next() => {
                            if let Ok(args) = signal.args() {
                                print_progress_bar_info("Building", args.package.as_str(), Color::Blue, Style::Bold);
                            }
                        },
                        Some(signal) = built.next() => {
                            if let Ok(args) = signal.args() {
                                print_progress_bar_info("Built", args.package.as_str(), Color::Blue, Style::Bold);
                                inc_progress_bar();
                            }
                        },
                        Some(signal) = failed.next() => {
                            if let Ok(args) = signal.args() {
                                print_progress_bar_info("Failed to build", args.package.as_str(), Color::Red, Style::Bold);
                            }
                        },
                        Some(_) = finished.next() => {
                            print_progress_bar_info("Finished", "installing AUR updates", Color::Green, Style::Bold);
                            break;
                        }
                        Some(_) = failure.next() => {
                            print_progress_bar_info("Failed", "installing AUR updates", Color::Red, Style::Bold);
                            break;
                        }
                        else => break
                    }
                }

                Ok(()) as zbus::Result<()>
            }
        };
        set_progress_style(ProgressStyle::Percentage);
        init_progress_bar(0);
        if let Some(size) = termsize::get() {
            set_progress_bar_width(size.cols as usize - 60);
            if size.cols > 200 {
                set_action_width(30);
            }
        }

        let mut download_started = false;

        loop {
            select! {
                Some(signal) = download.next() => {
                    if let Ok(args) = signal.args() {
                        if !download_started {
                            download_started = true;
                            print_progress_bar_info("Downloading", args.progress.filename.as_str(), Color::Green, Style::Bold);
                        }
                        if args.progress.status == args.progress.total {
                            print_progress_bar_info("Downloaded", args.progress.filename.as_str(), Color::Green, Style::Bold);
                            download_started = false;
                        }
                        set_progress_bar_max(args.progress.total as usize);
                        set_progress_bar_action(args.progress.filename.as_str(), Color::Blue, Style::Bold);
                        inc_progress_bar();
                    }
                },
                Some(signal) = update.next() => {
                    if let Ok(args) = signal.args() {
                        if args.progress.current == 0 {
                            set_progress_bar_max(args.progress.howmany);
                            set_progress_bar_progress(0);
                        }
                        set_progress_bar_action(args.progress.package.as_str(), Color::Blue, Style::Bold);
                        set_progress_bar_progress(args.progress.current);
                    }
                },
                Some(_) = finished.next() => {
                    print_progress_bar_info("Finished", "installing updates", Color::Green, Style::Bold);
                    if aur_packages_count > 0 {
                        update_aur().await?;
                    }
                    finalize_progress_bar();
                    break;
                }
                Some(_) = failure.next() => {
                    print_progress_bar_info("Failed", "installing updates", Color::Red, Style::Bold);
                    finalize_progress_bar();
                    break;
                }
                else => break
            }
        }

        Ok(())
    };

    let _ = screen_saver_proxy.uninhibit(cookie).await;

    res
}
