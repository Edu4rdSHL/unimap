# Unimap
Scan only once by IP address and reduce scan times with Nmap for large amounts of data. Unimap is an abbreviation of "Unique Nmap *Scan*". The tool can run in Linux, OSX, Windows or Android (Termux) without problems.

# Why?
If you have plans to run a Nmap to a whole organization you need to consider that surely tens, hundreds or even thousands of subdomains will point to the same IP address and there will come a point where it becomes almost impossible to continue scanning. Also your IP may end up blocked due to multiple scans to the same remote host address among other things.

# How?
Unimap uses its own technology to initially resolve the IP addresses of all subdomains, once this process is finished, it creates a vector with the unique IP addresses and launches a parallel scan with Nmap according to the number of threads that the user has configured, at the same time, it analyzes the data from the files created with Nmap to find out which information corresponds to each IP. Finally, Unimap relates the information of each of the IPs associated with the subdomains. So, for example, if you have 50 subdomains that point to the same IP, you will only do one Nmap scan but you will have all the data associated with each of the subdomains at the same time, in a large scan it saves days or weeks.

# Installation

## From source

You need to have Rust and Nmap installed in your computer, then run:

```
1. git clone https://github.com/Edu4rdSHL/unimap.git && cd unimap
# Alternatively you can download a release from https://github.com/Edu4rdSHL/unimap/releases/latest
# extract it and continue to next step.
2. cargo build --release
# Now the binary is in ./target/release/unimap
```

## Using precompiled binaries

Download the [latest version](https://github.com/Edu4rdSHL/unimap/releases/latest) for your OS and use it.

## Using the AUR packages. (Arch Linux)

`unimap` can be installed from available [AUR packages](https://aur.archlinux.org/packages/?O=0&SeB=b&K=unimap&outdated=&SB=n&SO=a&PP=50&do_Search=Go) using a [AUR helper](https://wiki.archlinux.org/index.php/AUR_helpers). For example,

```
$ paru -S unimap
```

If you prefer, you can clone the [AUR packages](https://aur.archlinux.org/packages/?O=0&SeB=b&K=unimap&outdated=&SB=n&SO=a&PP=50&do_Search=Go) and then compile them with [makepkg](https://wiki.archlinux.org/index.php/Makepkg). For example,

```
git clone https://aur.archlinux.org/unimap.git && cd unimap && makepkg -si
```

# Usage

**Unimap requires root/administrator privileges to launch [Nmap TCP SYN (Stealth) Scan](https://nmap.org/book/synscan.html), we use it for accuracy and performance reasons.** If you are on Linux or Linux-based, just use a root shell or run the tool with sudo, in Windows you can open a Command Prompt (CMD) as Administrator and run the tool as usual.

# Examples

1. `unimap -f targets.txt -u log.csv` performs a full scan and writes output to log.csv.
2. `unimap -f targets.txt --fast-scan -o` performs a fast scan and saves the logfile to the logs/ folder.
3. `sudo unimap -f targets.txt --ports "1-1000" --min-rate 5000` scans ports from 1-1000 doing service and version detection (if you want a fast scan use the --fast-scan flag) with a min-rate of 5000.

# Considerations

* Unimap is preconfigured to run on faster networks (cloud VPS), if you run a scan in a home network that doesn't have too much capacity you will end up disconnected due to network throttling.
* The previously doesn't mean you can not use Unimap from your home, just adjust the number of `--threads` and `--min-rate` (being it the most important).
* We do not wrap Nmap in any way or scan ports on our own, we use the right Nmap options to get the most performance, Nmap rocks and it's the fastest port scanner that currently exists.
* We parse Nmap output data and give you more understandable output while also preventing you to scan the same IP several times, **it's our main goal** .

# Found a bug?
Open an [issue](https://github.com/Edu4rdSHL/unimap).
