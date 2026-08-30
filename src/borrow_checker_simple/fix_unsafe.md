# Fixing an unsafe program

## Returning a reference to the Stack
1. Data must **outlive all of its references**. This means we should not return the pointer (in the stack). 