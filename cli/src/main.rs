use mytex::load_command_config;

fn main() -> anyhow::Result<()> {
    let configs = load_command_config(None)?;
    // dbg!(&configs);
    let converter = mytex::CommandLatexConverter { configs: &configs };

    let root = mytex::parse_to_tree(
        r"
        # hello
        this is a plain text
        # world
        $$
         f(n) &= (x+1)^2+(y+3)^2
         &= x^2 + 2x + 1 + y^2 + 6y + 9
         &= x^2 + y^2 + 2x + 6y + 10
        ",
        &configs,
    )?;

    let latex = converter.compile_command_into_latex(&root)?;

    println!("{:?}", root);
    println!("{}", latex);
    Ok(())
}
