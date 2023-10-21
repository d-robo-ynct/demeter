use safe_drive::{
    context::Context, error::DynError, logger::Logger, msg::common_interfaces::std_msgs,pr_info,
};

fn main() -> Result<(), DynError>{
    let ctx = Context::new()?;
    let node = ctx.create_node("demeter", None, Default::default())?;
    let subscriber = node.create_subscriber::<std_msgs::msg::Int8>("demeter_oracle", None)?;
    let publisher = node.create_publisher::<drobo_interfaces::msg::MdLibMsg>("md_driver_topic", None)?;
    let mut pub_msg = drobo_interfaces::msg::MdLibMsg::new().unwrap();
    pub_msg.address = 0x04;

    let logger = Logger::new("demeter");

    let mut selector = ctx.create_selector()?;
    selector.add_subscriber(
        subscriber, 
        Box::new(move |msg| {
            pub_msg.mode = if msg.data != 0 {5} else {2};
            pub_msg.phase = if msg.data <= 0 {true} else {false};
            pub_msg.power = if msg.data != 0 {999} else {0};
            pub_msg.port = if msg.data >= 0 {false} else {true};
            pub_msg.timeout = 100;
            pr_info!(logger, "収穫機構: {}", if msg.data == 1 {"上昇"} else if msg.data == 0 {"ストップ"} else {"下降"});
            publisher.send(&pub_msg).unwrap();
            if msg.data == -1 {
                publisher.send(&pub_msg).unwrap();
            }
        }),
    );

    loop {
        selector.wait()?;
    }
}
