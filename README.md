# DiskRecorder
A tool to record the changes of files in disk to analyze abnormal disk usage growth

## Design
1. Scan the Disk when the system starts or per 6 hours.
2. Scans the disk for files and directories and records the corresponding size recursively.
3. Record the difference between 2 scans
4. Graphical display of file system size changes between scan intervals.
5. Analyze the files that have the greatest impact on changes in file system size
6. Trigger an warning when the file system volume changes above a threshold value