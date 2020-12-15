# Serenity Benchmark
Check out the [GitHub Repository for Serenity](https://github.com/serenity-rs/serenity)

## My Benchmark
### Tested on
* **OS**: Ubuntu 20.10 x86_64
* **CPU**: Intel i5-7400 (4) @ 3.500GHz
* **GPU**: NVIDIA GeForce GTX 1050 Ti
### Results
Results while the bot was idling (1 guild - no messages received)
#### **PMAP**
```
total            89652K
```
#### **Massif**
```
--------------------------------------------------------------------------------
  n        time(i)         total(B)   useful-heap(B) extra-heap(B)    stacks(B)
--------------------------------------------------------------------------------
 88    506,084,203          430,600          410,649        19,951            0
 89    510,245,580          430,600          410,649        19,951            0
 90    514,308,707          430,600          410,649        19,951            0
 91    518,393,305          430,600          410,649        19,951            0
```

### How to benchmark
If you are using Linux you can benchmark using the following commands

1. Build the BOT using `cargo build`
2. Select one of both and follow the steps

#### **PMAP**
3. Run `DISCORD_TOKEN=<token> ./target/debug/discord-bot-benchmark`
4. Get the PID
5. Run `pmap <pid> | tail -n 1`
#### **Massif** with Valgrind
3. Run `DISCORD_TOKEN=<token> valgrind --tool=massif ./target/debug/discord-bot-benchmark`
4. Close the program (`CTRL+C`)
5. Run `ms_print massif.out.<pid> | tail -n 7`