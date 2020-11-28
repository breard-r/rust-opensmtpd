use crate::parsers::entry::{parse_entry, EntryOption};
use crate::parsers::parameters::{
    parse_filter_auth, parse_filter_connect, parse_filter_data_line, parse_filter_ehlo,
    parse_filter_helo, parse_filter_mail_from, parse_filter_rcpt_to, parse_filter_starttls,
    parse_report_filter_report, parse_report_filter_response, parse_report_link_auth,
    parse_report_link_connect, parse_report_link_greeting, parse_report_link_identify,
    parse_report_link_tls, parse_report_protocol_client, parse_report_protocol_server,
    parse_report_tx_begin, parse_report_tx_commit, parse_report_tx_data, parse_report_tx_envelope,
    parse_report_tx_mail, parse_report_tx_rcpt, parse_report_tx_reset, parse_report_tx_rollback,
};
use crate::{Event, Filter, FilterPhase};

macro_rules! handle_reports {
    ($obj: ident, $r: ident, $input: ident) => {
        match $r.event {
            Event::LinkAuth => {
                let (_, (username, result)) =
                    parse_report_link_auth($input).map_err(|e| e.to_string())?;
                $obj.on_report_link_auth(&$r, &username, result);
            }
            Event::LinkConnect => {
                let (_, (rdns, fcrdns, src, dest)) =
                    parse_report_link_connect($input).map_err(|e| e.to_string())?;
                $obj.on_report_link_connect(&$r, &rdns, &fcrdns, &src, &dest);
            }
            Event::LinkDisconnect => {
                $obj.on_report_link_disconnect(&$r);
            }
            Event::LinkGreeting => {
                let (_, hostname) =
                    parse_report_link_greeting($input).map_err(|e| e.to_string())?;
                $obj.on_report_link_greeting(&$r, &hostname);
            }
            Event::LinkIdentify => {
                let (_, (method, identity)) =
                    parse_report_link_identify($input).map_err(|e| e.to_string())?;
                $obj.on_report_link_identify(&$r, method, &identity);
            }
            Event::LinkTls => {
                let (_, s) = parse_report_link_tls($input).map_err(|e| e.to_string())?;
                $obj.on_report_link_tls(&$r, &s);
            }
            Event::TxBegin => {
                let (_, id) = parse_report_tx_begin($input).map_err(|e| e.to_string())?;
                $obj.on_report_tx_begin(&$r, &id);
            }
            Event::TxMail => {
                let (_, (id, result, addr)) =
                    parse_report_tx_mail($input).map_err(|e| e.to_string())?;
                $obj.on_report_tx_mail(&$r, &id, result, &addr);
            }
            Event::TxReset => {
                let (_, id) = parse_report_tx_reset($input).map_err(|e| e.to_string())?;
                $obj.on_report_tx_reset(&$r, &id);
            }
            Event::TxRcpt => {
                let (_, (id, result, addr)) =
                    parse_report_tx_rcpt($input).map_err(|e| e.to_string())?;
                $obj.on_report_tx_rcpt(&$r, &id, result, &addr);
            }
            Event::TxEnvelope => {
                let (_, (msg, env)) =
                    parse_report_tx_envelope($input).map_err(|e| e.to_string())?;
                $obj.on_report_tx_envelope(&$r, &msg, &env);
            }
            Event::TxData => {
                let (_, (id, result)) = parse_report_tx_data($input).map_err(|e| e.to_string())?;
                $obj.on_report_tx_data(&$r, &id, result);
            }
            Event::TxCommit => {
                let (_, (id, size)) = parse_report_tx_commit($input).map_err(|e| e.to_string())?;
                $obj.on_report_tx_commit(&$r, &id, size);
            }
            Event::TxRollback => {
                let (_, id) = parse_report_tx_rollback($input).map_err(|e| e.to_string())?;
                $obj.on_report_tx_rollback(&$r, &id);
            }
            Event::ProtocolClient => {
                let (_, cmd) = parse_report_protocol_client($input).map_err(|e| e.to_string())?;
                $obj.on_report_protocol_client(&$r, &cmd);
            }
            Event::ProtocolServer => {
                let (_, res) = parse_report_protocol_server($input).map_err(|e| e.to_string())?;
                $obj.on_report_protocol_server(&$r, &res);
            }
            Event::FilterResponse => {
                let (_, (phase, res, param)) =
                    parse_report_filter_response($input).map_err(|e| e.to_string())?;
                $obj.on_report_filter_response(&$r, phase, &res, &param);
            }
            Event::FilterReport => {
                let (_, (kind, name, message)) =
                    parse_report_filter_report($input).map_err(|e| e.to_string())?;
                $obj.on_report_filter_report(&$r, kind, &name, &message);
            }
            Event::Timeout => {
                $obj.on_report_timeout(&$r);
            }
        }
    };
}

macro_rules! handle_filters {
    ($obj: ident, $f: ident, $input: ident) => {
        match $f.phase {
            FilterPhase::Auth => {
                let (_, auth) = parse_filter_auth($input).map_err(|e| e.to_string())?;
                Some($obj.on_filter_auth(&$f, &auth))
            }
            FilterPhase::Commit => Some($obj.on_filter_commit(&$f)),
            FilterPhase::Connect => {
                let (_, (rdns, fcrdns, src, dest)) =
                    parse_filter_connect($input).map_err(|e| e.to_string())?;
                Some($obj.on_filter_connect(&$f, &rdns, &fcrdns, &src, &dest))
            }
            FilterPhase::Data => Some($obj.on_filter_data(&$f)),
            FilterPhase::DataLine => {
                let (_, data_line) = parse_filter_data_line($input).map_err(|e| e.to_string())?;
                $obj.on_filter_data_line(&$f, &data_line);
                None
            }
            FilterPhase::Ehlo => {
                let (_, identity) = parse_filter_ehlo($input).map_err(|e| e.to_string())?;
                Some($obj.on_filter_ehlo(&$f, &identity))
            }
            FilterPhase::Helo => {
                let (_, identity) = parse_filter_helo($input).map_err(|e| e.to_string())?;
                Some($obj.on_filter_helo(&$f, &identity))
            }
            FilterPhase::MailFrom => {
                let (_, address) = parse_filter_mail_from($input).map_err(|e| e.to_string())?;
                Some($obj.on_filter_mail_from(&$f, &address))
            }
            FilterPhase::RcptTo => {
                let (_, address) = parse_filter_rcpt_to($input).map_err(|e| e.to_string())?;
                Some($obj.on_filter_rcpt_to(&$f, &address))
            }
            FilterPhase::StartTls => {
                let (_, tls_str) = parse_filter_starttls($input).map_err(|e| e.to_string())?;
                Some($obj.on_filter_starttls(&$f, &tls_str))
            }
        }
    };
}

pub(crate) fn line<T>(user_object: &mut T, input: &[u8]) -> Result<(), String>
where
    T: Filter,
{
    let (input, entry) = parse_entry(input).map_err(|e| e.to_string())?;
    match entry {
        EntryOption::Report(r) => handle_reports!(user_object, r, input),
        EntryOption::Filter(f) => {
            if let Some(answer) = handle_filters!(user_object, f, input) {
                println!(
                    "filter-result|{}|{}|{}",
                    f.session_id,
                    f.token,
                    answer.to_string()
                );
            };
        }
    };
    Ok(())
}
