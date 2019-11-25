
struct Command {
    
}

struct Result {

}

trait CommandHandler {
    fn type() -> String;

    fn handle(cmd: Command) -> Result;
}
