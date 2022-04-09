use nu_protocol::{Config, PipelineData, Span};
use nupro::delimited::from_delimited_data;

fn main() {
    let config: Config = Config::default();
    let span: Span = Span::test_data();
    let input: PipelineData = PipelineData::new(span);
    let result = from_delimited_data(false, ',', input, span, &config);
    println!("{:?}", result);
}
