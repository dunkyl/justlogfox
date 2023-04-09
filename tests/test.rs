#[test]
fn all_public() {
    justlogfox::set_log_level(justlogfox::LogLevel::Trace);

    justlogfox::log!([justlogfox::unittest] justlogfox::LogLevel::Error, "fmt {}", "test");
    justlogfox::log!([justlogfox::unittest] justlogfox::LogLevel::Error, "test");
    justlogfox::log!(justlogfox::LogLevel::Error, "fmt {}", "test");
    justlogfox::log!(justlogfox::LogLevel::Error, "test");

    justlogfox::log_error!([justlogfox::unittest] "fmt {}", "test");
    justlogfox::log_error!([justlogfox::unittest] "test");
    justlogfox::log_error!("fmt {}", "test");
    justlogfox::log_error!("test");

    justlogfox::log_warn!([justlogfox::unittest] "fmt {}", "test");
    justlogfox::log_warn!([justlogfox::unittest] "test");
    justlogfox::log_warn!("fmt {}", "test");
    justlogfox::log_warn!("test");

    justlogfox::log_info!([justlogfox::unittest] "fmt {}", "test");
    justlogfox::log_info!([justlogfox::unittest] "test");
    justlogfox::log_info!("fmt {}", "test");
    justlogfox::log_info!("test");

    justlogfox::log_debug!([justlogfox::unittest] "fmt {}", "test");
    justlogfox::log_debug!([justlogfox::unittest] "test");
    justlogfox::log_debug!("fmt {}", "test");
    justlogfox::log_debug!("test");

    justlogfox::log_trace!([justlogfox::unittest] "fmt {}", "test");
    justlogfox::log_trace!([justlogfox::unittest] "test");
    justlogfox::log_trace!("fmt {}", "test");
    justlogfox::log_trace!("test");
}