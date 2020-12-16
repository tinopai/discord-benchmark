# discord.js Benchmark
Check out the [GitHub Repository for discord.js](https://github.com/discordjs/discord.js)

## My Benchmark
### Tested on
* **OS**: Ubuntu 20.10 x86_64
* **CPU**: Intel i5-7400 (4) @ 3.500GHz
* **GPU**: NVIDIA GeForce GTX 1050 Ti
### Results
Results while the bot was idling (1 guild - no messages received)
#### **Smaps: Pss**
```
MEM Usage: 42,4375 MB 
```
#### **PMAP**
```
total           820280K
```
#### **Massif**
```
--------------------------------------------------------------------------------
  n        time(i)         total(B)   useful-heap(B) extra-heap(B)    stacks(B)
--------------------------------------------------------------------------------
 87    704,090,550        3,460,344        3,133,737       326,607            0
 88    709,465,889        3,473,976        3,147,298       326,678            0
 89    714,968,213        3,474,776        3,148,050       326,726            0
 90    720,335,221        3,482,912        3,155,844       327,068            0
 91    725,702,688        3,523,432        3,195,911       327,521            0
 92    731,133,755        3,618,480        3,290,626       327,854            0
```

### How to benchmark
If you are using Linux you can benchmark using the following commands

1. Build the BOT using `cargo build`
2. Select one of both and follow the steps
### **Smaps: Pss**
3. Run `DISCORD_TOKEN=<token> node main.js`
4. Get the PID
5. Run `cat /proc/<pid>/smaps | grep -i pss |  awk '{Total+=$2} END {print Total/1024" MB"}'`

#### **PMAP**
3. Run `DISCORD_TOKEN=<token> node main.js`
4. Get the PID
5. Run `pmap <pid> | tail -n 1`
#### **Massif** with Valgrind
3. Run `DISCORD_TOKEN=<token> valgrind --tool=massif node main.js`
4. Close the program (`CTRL+C`)
5. Run `ms_print massif.out.<pid> | tail -n <number: usually 7>`