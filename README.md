## What is R Packman?
The purposes of this R script are to download all R packages from CRAN repository and configure local repostiory to use it in dark site where internet could not be connected.
After configuring local repostory you could install and uninstall packages from local repository firstl.
In case there are no packages in local repository this script would connect automatically into CRAN repository.

## Supported and confirmed OS versoins so far
RHEL, CentOS 7.x
RHEL, Rocky Linux 8 and 9.x

## How to configure local repository
### The following command would create src/contrib directory under current directory and then download all packages and metadata, PACKAGES from CRAN repository.
~~
$ ./r-packman.rs -o download -p all
~~

## How to use it
~~~
$ ./r-packman.rs -h
[1] "Dependent packages are already installed or downloaded on local repository"
Usage: ./r-packman.rs -o <install|uninstall> -p <package name>

$ ./r-packman.rs -v
[1] "************************** Setup Dependency Packages *************************"
[1] "Dependent packages are already installed or downloaded on local repository"

     For examples how to use this proglem,

     1) Download all Packages from CRAN Repository
     $ ./r-packman.rs -o download -p all

     2) Download the specfic Packages from CRAN Repository
     $ ./r-packman.rs -o download -p ipred

     3) Install the specfic Packages from direcotry where packages were downloaded
     $ ./r-packman.rs -o install -p ipred

     4) Uninstall the specific packages installed
     $ ./r_packman.rs -o uninstall -p ipred

     5) Uninstall all packages installed
     $ ./r_packman.rs -o uninstall -p all
~~~

## Refrences
* https://docs.posit.co/resources/install-r/
* https://docs.posit.co/rpm/installation/
* https://docs.posit.co/rspm/admin/repositories/#multiple-sources
* ttps://docs.posit.co/rspm/admin/appendix/system-dependency-detection/
* https://github.com/rstudio/r-builds
* https://www.rdocumentation.org/packages/utils/versions/3.6.2/topics/install.packages
* https://cran.r-project.org/bin/windows/base/howto-R-devel.html
* https://cran.r-project.org/doc/manuals/r-release/R-admin.html
* https://enchufa2.github.io/rspm/
