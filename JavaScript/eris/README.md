# Eris Benchmark
Check out the [GitHub Repository for Eris](https://github.com/abalabahaha/eris)

## My Benchmark
### Tested on
* **OS**: Ubuntu 20.10 x86_64
* **CPU**: Intel i5-7400 (4) @ 3.500GHz
* **GPU**: NVIDIA GeForce GTX 1050 Ti
### Results
Results while the bot was idling (1 guild - no messages received)
#### **Smaps: Pss**
```
MEM Usage: 37,9082 MB 
```
#### **PMAP**
```
total           748264K
```
#### **Massif**
```
--------------------------------------------------------------------------------
  n        time(i)         total(B)   useful-heap(B) extra-heap(B)    stacks(B)
--------------------------------------------------------------------------------
 79    427,339,606        3,040,456        2,731,578       308,878            0
 80    432,040,043        3,050,056        2,740,989       309,067            0
 81    436,497,564        3,346,752        3,037,513       309,239            0
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