
# Problem

People often need to use multiple programs to create files especially for research purposes. These created files are then used to create further files creating a adhoc pipelined workflow. Sometimes the manipulation is done all within the same program making the workflow simple. However, often for smaller more hand-crafted programs this is not the case. Users often want to load a file perform some manipulation either interactive or non-interactive. If the manipulation is interactive this can be either CLI or GUI based. Often these transformations are performed one remote machine as the command is long running or/and requires large amounts of resources. These files can be manipulated and shared amongst multiple people working typically in a team. This is typically done by users sharing a directory on a remote machine. The size of these files can be large.


![alt](./out/diagrams/domaim%20model.png)

# Solution

Control the creation of files within a pipeline. Users create configuration files that specify the actions take for manipulation. Then users from given inputs can select the desired manipulation. The configuration specifies the file extensions it can work on. For simplicity the program will run on a single machine likely a remote machine. The current view will be through a CLI.


## Use cases

### To Be Implemented
User creates configuration file and puts the file inside the configuration folder to be loaded in by the applicaiton. 

User selects input files and selects program to transform them. If the program is interactive the CLI allows for manipulation. The program creates the new files



### Not Implemented

Interactive method to create a configuration file for a specified program

Interactive Graphical programs will not be directly supported. Since this requires GUI that is unlikely on remote machines. A X11 Window can be used through SSH to achieve this.

