use std::process::Stdio;

use ffmpeg_cli::{FfmpegBuilder, File, Parameter};

pub async fn process_file(input: &str, output: &str) {
    let builder = FfmpegBuilder::new()
        .stderr(Stdio::piped())
        .option(Parameter::Single("nostdin"))
        .option(Parameter::Single("y"))
        .input(File::new(input))
        .output(
            File::new(output)
                .option(Parameter::KeyValue("vcodec", "libx264"))
                .option(Parameter::KeyValue("acodec", "aac")),
        );

    let ffmpeg = builder.run().await.unwrap();

    let output = ffmpeg.process.wait_with_output().unwrap();

    println!(
        "{}\nstderr:\n{}",
        output.status,
        std::str::from_utf8(&output.stderr).unwrap()
    );
}
