use ws::{connect, Result, Handler, Message, Sender};
use ws::util::Token;

const URL: &'static str ="wss://mastodon.xyz/api/v1/streaming/?stream=public";
const RESET_CONN: Token = Token(1024);
    
struct DemoHandler {
    out: Sender,
}

impl Handler for DemoHandler {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("MSG = {:?}", msg);
        Ok(())
    }

    fn on_timeout(&mut self, event: Token) -> Result<()> {
        if event == RESET_CONN {
            println!("RESET");
            self.out.shutdown().unwrap();
        }
        Ok(())
    }
}

fn main() {
    loop {
        connect(URL, |out| {
            out.timeout(60000, RESET_CONN).unwrap();
            DemoHandler{out}
        }).unwrap();
    }
}
