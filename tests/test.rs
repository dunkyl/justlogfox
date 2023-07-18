#[test]
fn all_public() {
    use justlogfox::*;

    set_log_level(LogLevel::Trace);

    log!([justlogfox::test] LogLevel::Error, "fmt {}", "test");
    log!([justlogfox::test] LogLevel::Error, "test");
    log!(LogLevel::Error, "fmt {}", "test");
    log!(LogLevel::Error, "test");

    log_error!([justlogfox::test] "fmt {}", "test");
    log_error!([justlogfox::test] "test");
    log_error!("fmt {}", "test");
    log_error!("test");

    log_warn!([justlogfox::test] "fmt {}", "test");
    log_warn!([justlogfox::test] "test");
    log_warn!("fmt {}", "test");
    log_warn!("test");

    log_info!([justlogfox::test] "fmt {}", "test");
    log_info!([justlogfox::test] "test");
    log_info!("fmt {}", "test");
    log_info!("test");

    log_debug!([justlogfox::test] "fmt {}", "test");
    log_debug!([justlogfox::test] "test");
    log_debug!("fmt {}", "test");
    log_debug!("test");

    log_trace!([justlogfox::test] "fmt {}", "test");
    log_trace!([justlogfox::test] "test");
    log_trace!("fmt {}", "test");
    log_trace!("test");
}