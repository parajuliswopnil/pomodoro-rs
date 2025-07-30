To use the pomodoro clock, you will first need to install beeping binary from 

```bash 
git clone https://github.com/RustAudio/cpal
cd cpal 
cargo build --release --examples 
cp target/release/examples/beep ~/.cargo/bin
```

Source your shell 
```bash 
source ~/.zshrc # for zsh
```
```bash
source ~/.bashrc # for bash
```

Now from the root of this project, 
```bash
cargo build --release 
cp target/release/pomodoro ~/.cargo/bin
```

Source your shell 
```bash 
source ~/.zshrc # for zsh
```
```bash
source ~/.bashrc # for bash
```

Now to use the clock 
```bash 
pomodoro
```
By default work time is set to 20 minutes and rest is set to 5 minutes which repeats for only 1 cycle

To use custom settings 
```bash
pomodoro --work <minutes> --rest <minutes> --cycle <number>
```