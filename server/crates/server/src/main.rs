use proto_rs::{controller, host};

fn main() {
    let host_request = host::HostRequest {
        msg: Some(host::host_request::Msg::Ping(host::PingMessage {
            msg: "ping from server to host".to_string(),
        })),
    };

    let controller_request = controller::ControllerRequest {
        msg: Some(controller::controller_request::Msg::Ping(
            controller::PingMessage {
                msg: "ping from server to controller".to_string(),
            },
        )),
    };

    let host_response = host::ServerResponse {
        msg: Some(host::server_response::Msg::Pong(host::PongMessage {
            msg: "pong from host".to_string(),
        })),
    };

    let host_ping_text = match host_request.msg.as_ref() {
        Some(host::host_request::Msg::Ping(ping)) => ping.msg.as_str(),
        _ => "<unexpected host message>",
    };

    let controller_ping_text = match controller_request.msg.as_ref() {
        Some(controller::controller_request::Msg::Ping(ping)) => ping.msg.as_str(),
        _ => "<unexpected controller message>",
    };

    let host_pong_text = match host_response.msg.as_ref() {
        Some(host::server_response::Msg::Pong(pong)) => pong.msg.as_str(),
        _ => "<unexpected host response>",
    };

    println!("Host export test: {host_ping_text}");
    println!("Controller export test: {controller_ping_text}");
    println!("Host response export test: {host_pong_text}");
}
