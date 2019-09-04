if [[ $# -ne 1 ]]; then
    echo "Usage: `basename $0` <DAY_NUMBER>"
    exit 1
fi

filename_src=$(printf "./src/bin/day%02d.rs" $1)
filename_data=$(printf "./data/day%02d.txt" $1)

cat << RUST > $filename_src
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
    }

    Ok(())
}
RUST

touch $filename_data
