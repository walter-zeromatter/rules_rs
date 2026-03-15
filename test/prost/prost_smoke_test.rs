use echo_proto::echo::EchoRequest;

#[test]
fn generated_message_is_constructible() {
    let request = EchoRequest {
        message: "hello".to_owned(),
    };

    assert_eq!(request.message, "hello");
}
