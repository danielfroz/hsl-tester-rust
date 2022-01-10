# Introduction

This is a simple example of F5 HSL service, which can be used for debugging purpose.
This is still a working progress...

# How to use it?

Deploy the package or build from source ... once you have the ```hsl-tester``` on your target linux or macos
then just use the following commands:

```
$ ./hsl-tester 0.0.0.0:11111 -
```
Note: command above is instructing to listen to UDP on all interfaces on port 11111. 
There is no real need for TCP implementation. So far, only UDP option is available.

It will print all the HSL messages to the STDOUT. If you want to inspect the messages, just either use tee
or instead of using dash as last argument, add the file path / filename...

```
$ ./hsl-tester 0.0.0.0:11111 output.log
```