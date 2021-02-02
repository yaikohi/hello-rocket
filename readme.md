
# Goal
To learn about rust programming, rocker, setting up API's, docker, CLI, diesel.

## Resources
I have been following this [youtube tutorial](https://www.youtube.com/watch?v=tjH0Mye8U_A). Following the tutorial I look up for solutions and/or issues I had trying to solve them before continuing the tutorial.

I am also using the rocker documentation and the official rust book for package or code-related problems.



# NOTES
While working on this small project I had some issues and some notes I wanted to write down.

### ISSUE nr 1: 
Trying to install diesel_cli just like on the youtube video provided me with an error.
(copy paste from the terminal window)
```
error: 
error: aborting due to previous error
error: failed to compile `diesel_cli v1.4.1`, intermediate artifacts can be found at `C:\Users\ryuza\AppData\Local\Temp\cargo-installbZpw3s`
```
#### SOLUTION: 
Turns out the postgres lib file was missing from a rustup folder. It was fixed as follows.

copy libpq.dll & libpq.lib from the postgres/lib folder to /rustlib/.../.../.../lib.

SRC: [link to github issue](https://github.com/diesel-rs/diesel/issues/1883)
This solution worked for me and diesel_cli was installed properly with cargo install afterwards.


### ISSUE nr 2:
The tutorial uses the ```export``` command (```export DATABASE_URL=postgres://rocket:rocket@localhost/rocket```) 
but since I use windows and not linux this doesn't work for me.
#### SOLUTION:
I used the 'echo' command instead of 'export'. [src](https://superuser.com/questions/1500272/equivalent-of-export-command-in-windows)

solution found at the [official documentation page](https://diesel.rs/guides/getting-started/) (scroll down a bit).

### ISSUE nr 3:
The ```diesel setup``` command did not provide me with a generated 'migrations' folder in my project directory.
Using ```diesel``` in the terminal does not provide me with anything whatsoever. 
Trying to use the diesel command in a new cmd provided me the error of "LIBPQ.DLL NOT FOUND, INSTALL PROPERLY" so I'll download postgresql again and see if it fixes it. (This wasn't the solution.)

This wasn't the solution so I ended up googling a bit more.

### SOLUTION:
Adding both the postgresql bin and lib folders to my environment path.

### ISSUE nr 4: postgres
I have problems understanding docker and how postgres interacts with docker. It tells me the password for user rocket isn't rocket even though I specified this in the docker-compose.yml file. 

Since I've never actually used docker before I decided to look up some tutorials on this. 

I found [this](https://www.youtube.com/watch?v=Qw9zlE3t8Ko) short tutorial on how docker-compose.yml's work.

After watching this youtube video I 

