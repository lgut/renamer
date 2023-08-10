use rand::{distributions::Alphanumeric, rngs::ThreadRng, Rng};
use std::{
    env::{self},
    fs::{self},
    io::{self},
    path::{Path, PathBuf},
};
use strings::MISSING_POSITIONAL_ARGS_MSG;

mod options;
mod strings;
use crate::strings::HELP_MSG;
use options::Options;

fn main() -> Result<(), ()> {
    let args: Vec<String> = env::args().collect();
    let opts_res = Options::parse(&args);

    match opts_res {
        Err(l) => {
            help_err(&l);
            Err(())
        }
        Ok(opts) => {
            if opts.print_help {
                help();
                return Ok(());
            }

            let mut rng: ThreadRng = rand::thread_rng();

            let src: &PathBuf = opts
                .source_path
                .as_ref()
                .expect(MISSING_POSITIONAL_ARGS_MSG);

            let io_res = match opts.dest_path.as_ref() {
                Some(dest) => copy_routine(&opts, src, dest, &mut rng),
                None => copy_routine(&opts, src, &Path::new(""), &mut rng),
            };

            match io_res {
                Err(l) => {
                    help_err(&l.to_string());
                    Err(())
                }
                _ => Ok(()),
            }
        }
    }
}

fn copy_routine(
    opts: &Options,
    src: impl AsRef<Path>,
    dest: impl AsRef<Path>,
    rng: &mut ThreadRng,
) -> io::Result<()> {
    fs::create_dir_all(&dest)?;

    let sample: String = rng
        .sample_iter(&Alphanumeric)
        .take(opts.sample_size)
        .map(char::from)
        .collect();

    for entry in fs::read_dir(&src)? {
        let entry = entry?;
        let f_type = entry.file_type()?;

        if f_type.is_dir() {
            if opts.recursive {
                copy_routine(
                    opts,
                    entry.path(),
                    dest.as_ref().join(entry.file_name()),
                    rng,
                )?;
            }
        } else {
            let from = entry.path();
            let new_file_name = format!("{}_{}", sample, entry.file_name().to_string_lossy());

            if opts.in_place {
                let to = src.as_ref().join(new_file_name);
                fs::rename(&from, &to)?;
                if !opts.quite {
                    println!("{} -> {}", from.to_string_lossy(), to.to_string_lossy());
                }
            } else {
                let to = dest.as_ref().join(new_file_name);
                fs::copy(&from, &to)?;
                if !opts.quite {
                    println!("{} -> {}", from.to_string_lossy(), to.to_string_lossy());
                }
            }
        }
    }
    Ok(())
}

fn help() {
    println!("{}", HELP_MSG);
}
fn help_err(err_msg: &String) {
    eprintln!("{}\n\n{}", err_msg, HELP_MSG);
}
