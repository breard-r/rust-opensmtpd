use crate::{
    Address, AuthResult, FilterEntry, FilterKind, FilterPhase, FilterResponse, MailResult, Method,
    ReportEntry,
};

pub trait Filter {
    fn on_filter_auth(&mut self, _entry: &FilterEntry, _auth: &str) -> FilterResponse {
        FilterResponse::Proceed
    }
    #[doc(hidden)]
    fn has_filter_auth(&self) -> bool {
        false
    }

    fn on_filter_commit(&mut self, _entry: &FilterEntry) -> FilterResponse {
        FilterResponse::Proceed
    }
    #[doc(hidden)]
    fn has_filter_commit(&self) -> bool {
        false
    }

    fn on_filter_connect(
        &mut self,
        _entry: &FilterEntry,
        _rdns: &str,
        _fcrdns: &str,
        _src: &Address,
        _dest: &Address,
    ) -> FilterResponse {
        FilterResponse::Proceed
    }
    #[doc(hidden)]
    fn has_filter_connect(&self) -> bool {
        false
    }

    fn on_filter_data(&mut self, _entry: &FilterEntry) -> FilterResponse {
        FilterResponse::Proceed
    }
    #[doc(hidden)]
    fn has_filter_data(&self) -> bool {
        false
    }

    fn on_filter_data_line(&mut self, _entry: &FilterEntry, _data_line: &[u8]) {}
    #[doc(hidden)]
    fn has_filter_data_line(&self) -> bool {
        false
    }

    fn on_filter_ehlo(&mut self, _entry: &FilterEntry, _identity: &str) -> FilterResponse {
        FilterResponse::Proceed
    }
    #[doc(hidden)]
    fn has_filter_ehlo(&self) -> bool {
        false
    }

    fn on_filter_helo(&mut self, _entry: &FilterEntry, _identity: &str) -> FilterResponse {
        FilterResponse::Proceed
    }
    #[doc(hidden)]
    fn has_filter_helo(&self) -> bool {
        false
    }

    fn on_filter_mail_from(&mut self, _entry: &FilterEntry, _address: &str) -> FilterResponse {
        FilterResponse::Proceed
    }
    #[doc(hidden)]
    fn has_filter_mail_from(&self) -> bool {
        false
    }

    fn on_filter_rcpt_to(&mut self, _entry: &FilterEntry, _address: &str) -> FilterResponse {
        FilterResponse::Proceed
    }
    #[doc(hidden)]
    fn has_filter_rcpt_to(&self) -> bool {
        false
    }

    fn on_filter_starttls(&mut self, _entry: &FilterEntry, _tls_string: &str) -> FilterResponse {
        FilterResponse::Proceed
    }
    #[doc(hidden)]
    fn has_filter_starttls(&self) -> bool {
        false
    }

    fn on_report_link_auth(&mut self, _entry: &ReportEntry, _username: &str, _result: AuthResult) {}
    #[doc(hidden)]
    fn has_report_link_auth(&self) -> bool {
        false
    }

    fn on_report_link_connect(
        &mut self,
        _entry: &ReportEntry,
        _rdns: &str,
        _fcrdns: &str,
        _src: &Address,
        _dest: &Address,
    ) {
    }
    #[doc(hidden)]
    fn has_report_link_connect(&self) -> bool {
        false
    }

    fn on_report_link_disconnect(&mut self, _entry: &ReportEntry) {}
    #[doc(hidden)]
    fn has_report_link_disconnect(&self) -> bool {
        false
    }

    fn on_report_link_greeting(&mut self, _entry: &ReportEntry, _hostname: &str) {}
    #[doc(hidden)]
    fn has_report_link_greeting(&self) -> bool {
        false
    }

    fn on_report_link_identify(&mut self, _entry: &ReportEntry, _method: Method, _identity: &str) {}
    #[doc(hidden)]
    fn has_report_link_identify(&self) -> bool {
        false
    }

    fn on_report_link_tls(&mut self, _entry: &ReportEntry, _tls_string: &str) {}
    #[doc(hidden)]
    fn has_report_link_tls(&self) -> bool {
        false
    }

    fn on_report_tx_begin(&mut self, _entry: &ReportEntry, _message_id: &str) {}
    #[doc(hidden)]
    fn has_report_tx_begin(&self) -> bool {
        false
    }

    fn on_report_tx_mail(
        &mut self,
        _entry: &ReportEntry,
        _message_id: &str,
        _result: MailResult,
        _address: &str,
    ) {
    }
    #[doc(hidden)]
    fn has_report_tx_mail(&self) -> bool {
        false
    }

    fn on_report_tx_reset(&mut self, _entry: &ReportEntry, _message_id: &Option<String>) {}
    #[doc(hidden)]
    fn has_report_tx_reset(&self) -> bool {
        false
    }

    fn on_report_tx_rcpt(
        &mut self,
        _entry: &ReportEntry,
        _message_id: &str,
        _result: MailResult,
        _address: &str,
    ) {
    }
    #[doc(hidden)]
    fn has_report_tx_rcpt(&self) -> bool {
        false
    }

    fn on_report_tx_envelope(
        &mut self,
        _entry: &ReportEntry,
        _message_id: &str,
        _envelope_id: &str,
    ) {
    }
    #[doc(hidden)]
    fn has_report_tx_envelope(&self) -> bool {
        false
    }

    fn on_report_tx_data(&mut self, _entry: &ReportEntry, _message_id: &str, _result: MailResult) {}
    #[doc(hidden)]
    fn has_report_tx_data(&self) -> bool {
        false
    }

    fn on_report_tx_commit(
        &mut self,
        _entry: &ReportEntry,
        _message_id: &str,
        _message_size: usize,
    ) {
    }
    #[doc(hidden)]
    fn has_report_tx_commit(&self) -> bool {
        false
    }

    fn on_report_tx_rollback(&mut self, _entry: &ReportEntry, _message_id: &str) {}
    #[doc(hidden)]
    fn has_report_tx_rollback(&self) -> bool {
        false
    }

    fn on_report_protocol_client(&mut self, _entry: &ReportEntry, _command: &str) {}
    #[doc(hidden)]
    fn has_report_protocol_client(&self) -> bool {
        false
    }

    fn on_report_protocol_server(&mut self, _entry: &ReportEntry, _response: &str) {}
    #[doc(hidden)]
    fn has_report_protocol_server(&self) -> bool {
        false
    }

    fn on_report_filter_response(
        &mut self,
        _entry: &ReportEntry,
        _phase: FilterPhase,
        _response: &str,
        _param: &Option<String>,
    ) {
    }
    #[doc(hidden)]
    fn has_report_filter_response(&self) -> bool {
        false
    }

    fn on_report_filter_report(
        &mut self,
        _entry: &ReportEntry,
        _filter_kind: FilterKind,
        _name: &str,
        _message: &str,
    ) {
    }
    #[doc(hidden)]
    fn has_report_filter_report(&self) -> bool {
        false
    }

    fn on_report_timeout(&mut self, _entry: &ReportEntry) {}
    #[doc(hidden)]
    fn has_report_timeout(&self) -> bool {
        false
    }
}
