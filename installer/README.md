# Cage Runtime Dependencies Installer

The Cage runtime depends on the following packages:

- runit
- ifconfig

Typically, these would be installed using a package manager like `apk` or `apt-get`. However, to allow Cages builds to be reproducible we cannot use these package managers. Installing dependencies using a package manager generates internal files which throw off the reproducible builds by including things like timestamps (such as the scripts tar file generated by apk).

To get around using the package managers, we need our own consistent installer for the runtime's dependencies. To make it consistent, Cages will pull a pinned version of the installer (guaranteeing the tar file's timestamps won't change).

The archive includes precompiled binaries of the runtimes dependencies. These binaries are built in a container defined in `Dockerfile` which outputs the binaries and an installer script in an archive.

## Testing

The binaries and installer can be tested by building `test.Dockerfile`, and running the commands: runit, ifconfig
