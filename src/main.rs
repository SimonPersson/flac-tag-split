use metaflac::Tag;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "flac-tag-split")]
struct Args {
    // The delimiter that tags are separated by
    #[structopt(short = "d", long = "delimiter")]
    delimiter: String,

    // A field that should be split
    #[structopt(short = "f", long = "field")]
    fields: Vec<String>,

    // Files
    #[structopt(name = "FILE", parse(from_os_str))]
    files: Vec<PathBuf>,
}

fn split_option_seq(tag: &Tag, field: &str, delimiter: &str) -> Vec<String> {
    tag.get_vorbis(field)
        .into_iter()
        .flat_map(|vec| vec.iter())
        .map(|s| &**s)
        .flat_map(move |s| s.split(delimiter))
        .map(String::from)
        .collect()
}

fn main() -> Result<(), metaflac::Error> {
    let args = Args::from_args();
    for file in &args.files {
        let mut tag = Tag::read_from_path(file).unwrap();
        for field in &args.fields {
            let split_fields = split_option_seq(&tag, field, &*args.delimiter);
            tag.remove_vorbis(field);
            tag.set_vorbis(&**field, split_fields);
            tag.save()?;
        }
    }
    Ok(())
}
