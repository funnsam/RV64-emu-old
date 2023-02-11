pub struct Args {
    pub rom: Option<String>
}

pub fn parse(args: &mut std::env::Args) -> Args {
    args.next();

    let mut p = Args { rom: None };
    for i in args.into_iter() {
        p.rom = Some(i)
    }
    p
}