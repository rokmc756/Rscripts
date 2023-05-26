## What is R Packman and the purpose of it?
The purposes of this R script is to download all R packages from CRAN repository and configure local repostiory to use it in dark site where internet could not be connected.
After configuring local repostory you could install and uninstall packages from local repository firstly.
In case there are no packages in local repository this script would connect automatically into CRAN repository.

Additionally you could download,install and uninstall multiple packages with option specified by comman and listed in a file.

## Supported and confirmed OS versoins so far
RHEL, CentOS 7.x
RHEL, Rocky Linux 8 and 9.x

## How to configure local repository
#### The following command would create src/contrib directory under current directory and then download metadata, PACKAGES and all packages from mirror CRAN repository.
~~~
$ ./r-packman.rs -o download -p all
~~~

## How to use it
#### Download or Install or update or or unstall a package
~~~
$ ./r-packman.rs -o <download|install|update|uninstall> -p ipred
~~~
#### Download or Install or unistall multiple packages with option seperated by comma
~~~
$ ./r-packman.rs -o <download|install|update|uninstall> -p ipred,AAtools
~~~
#### Download or Install or update or unstall multiple packages described at a file
~~~
$ vi pkglist.csv
AalenJohansen
AATtools
ipred

$ ./r-packman.rs -o <download|install|uninstall> -p pkglist.csv
~~~

#### Simple usage
~~~
$ ./r-packman.rs -h
Usage: ./r-packman.rs -o <install|uninstall> -p <package name>
~~~

#### Detail usage
~~~
$ ./r-packman.rs -v
     For examples how to use this proglem,

     1) Download all Packages from CRAN Repository and configure local repository
     $ ./r-packman.rs -o download -p all

     2) Download or install or update or uninstall the specfic Packages from CRAN Repository
     $ ./r-packman.rs -o <download|install|update|uninstall> -p ipred

     3) Download or install or update or uninstall multiple Packages with option from CRAN Repository
     $ ./r-packman.rs -o <download|install|update|uninstall> -p ipred,AATools

     4) Download or install or update or uninstall multiple Packages with file from CRAN Repository
     $ vi pkglist.csv
     AalenJohansen
     AATtools
     ipred
     $ ./r-packman.rs -o <download|install|update|uninstall> -p pkglist.csv

     5) Uninstall all packages installed except base and recommended provided by linux or R destribution.
     $ ./r-packman.rs -o uninstall -p all
~~~

## Planning
#### Still checking which part should it be improvied

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
