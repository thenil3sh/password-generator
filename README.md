# Password Generator

Just a password generator, yep, thats it

## Try it, how?

Two ways, you can give it a try : 

### The release page 
An easy way to do it; head to the [release page](https://github.com/oreo3494/password-generator/releases). There, download the executable respective to your system. 
 - As a `Windows` user, you may want to choose either `32 bit` or a `64 bit`, `.exe` file... \
 Note that, your antivirus may show up with a warning whenever you execute this file, or moreover, antivirus could delete the file, as soon as it gets downloaded. To deal with it, you need to keep it disabled atleast for the duration you're running the executable.
 Just to make it clear, its not a virus/malware and this project wasn't ever built with such goals. \
 If you still don't trust the executables... you can try **Build from source** method instead. :D

 - `Linux` user? doesn't look like you need an explanation? do you? :0 \
 Your system may not allow you to execute it by default... However you can modify its permissions using `chmod`.\
 Here's an example command : `$ chmod 777 /path/to/binary/`\
 Ofcourse replace `/path/to/binary` with the address to executable.


### Build from Source 
1. Before we even start... make sure you've `Rustup` (`Rust`) installed on your system... Don't have it? [Rust Book](https://doc.rust-lang.org/stable/book/ch01-01-installation.html) lends you some help.\

2. Clone this repository : \
`git clone https://github.com/oreo3494/password-generator/`\
Don't have git installed : [git-scm.com](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)'s installation guide is here to save you.

3. Change your current directory to to `password-generator`\
`$ cd /path/to/password-generator`

4. Use `cargo`  to build an executable out its source code :\
`$ cargo build --release`

5. If building ends up successful, there should be an executable present at `/target/release` with name `password-generator`(`.exe` if you're on Windows).
