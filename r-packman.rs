#!/usr/bin/env Rscript
#
#
# VMwrae Tanzu Support for Daatabase
# Staff Product Support Engineer
# Jack, Moon <moonja@vmware.com>
#
# ChangeLog
# 2023-05-20 - Cloned initally from http://gpdbkr.blogspot.com/search/label/GPDB6%20R%EC%84%A4%EC%B9%98
# 2023-05-22 - Add getopt option and functions were divided by operations
# 2023-05-26 - Changed how to install from local directory where packages were downloaded

mainDir <- "./"
subDir <- "src/contrib"
dir.create(file.path(mainDir, subDir), showWarnings = FALSE)
originSite <- "https://cloud.r-project.org"
mirrorSite <- "https://cran.yu.ac.kr"
localSite <- "file:///tmp/nfsshare/r-pkgs"

#
getAllPkgs <- function() {

  print("************************** Get All Packages *************************")
  # Get the metadata of all packages
  destfile <- file.path(mainDir, paste(subDir,"/PACKAGES",sep=""))
  url <- paste(mirrorSite, paste("/",subDir,"/PACKAGES",sep=""),sep="")
  download.file(url, destfile)

  # Get all packages
  library("rvest")
  pkgs <- read_html(paste(mirrorSite,"web/packages/available_packages_by_name.html",sep="/"))
  tab <- html_nodes(pkgs, "table") %>% html_table(fill = TRUE)
  pkgnames <- tab[[1]][1]$X1
  pkgnames <- pkgnames[nchar(pkgnames)>0]

  options(repos = c(CRAN <- mirrorSite ))
  download.packages(pkgnames, destdir=file.path(mainDir, subDir))

}

depPkgs <- function() {

  print("************************** Setup Dependency Packages *************************")
  dep_packs <- c("getopt","rvest","tseries")
  not_installed <- dep_packs[!(dep_packs %in% installed.packages()[ , "Package"])]

  if ( length(not_installed) ) {
    ie <- install.packages(c(dep_packs), repos = localSite, dependencies = TRUE)
    tryCatch(
      stop(ie),
      warning = install.packages(c(dep_packs), repos = mirrorSite, dependencies = TRUE),
      error = install.packages(c(dep_packs), repos = mirrorSite, dependencies = TRUE),
      finally = install.packages(c(dep_packs), repos = originSite, dependencies = TRUE)
    )
  } else {
    print(paste("Dependent packages are already installed or downloaded on local repository"))
  }

}

#
infoPkgs <- function(packs) {

  print("************************** Print Packages Info *************************")
  packages <- unlist(
    # Find (recursively) dependencies or reverse dependencies of packages.
    tools::package_dependencies(packs, available.packages(), which=c("Depends", "Imports"), recursive=TRUE)
  )
  packages <- union(packs, packages)
  return(packages)

}

#
getPkgs <- function(packs) {

  print("************************** Download Packages *****************************")
  options(repos = c(CRAN <- mirrorSite))
  packages <- infoPackages(c(packs))
  download.packages(packages, destdir=file.path(mainDir, subDir))
  # download.packages(packages, destdir=file.path(mainDir, subDir), type="binary")

}

#
setPkgs <- function(packs) {

  print("************************** Install Packages *****************************")
  not_installed <- packs[!(packs %in% installed.packages()[ , "Package"])]         

  if ( length(not_installed) ) {
    ie <- install.packages(c(packs), repos = localSite, dependencies = TRUE)
    tryCatch(
      stop(ie),
      warning = install.packages(c(packs), repos = mirrorSite, dependencies = TRUE),
      error = install.packages(c(packs), repos = mirrorSite, dependencies = TRUE),
      finally = install.packages(c(packs), repos = originSite, dependencies = TRUE)
    )
  } else {
    print(paste("Packages are already installed or downloaded on local repository"))
  }

}

#
updPkgs <- function(packs) {

  print("************************** Update Packages *****************************")
  upd_packs <- packs
  installed <- upd_packs[(upd_packs %in% installed.packages()[ , "Package"])]

  #print("*********************************************************************")
  #update.packages(c(upd_packs), repos = mirrorSite, dependencies = TRUE)
  #print("*********************************************************************")

  if ( length(installed) ) {
    ue <- update.packages(c(upd_packs), repos = mirrorSite, dependencies = TRUE)
    tryCatch(
      stop(ue),
      warning = update.packages(c(upd_packs), repos = mirrorSite, dependencies = TRUE),
      error = update.packages(c(upd_packs), repos = originSite, dependencies = TRUE),
      finally = print("Completed to update packages")
    )
  } else {
    print("Packages are already updated")
  }

}


#
rmPkgs <- function(packs) {

  print("************************** Uninstall Packages  *****************************")
  remove.packages( packs )

}


# Remove all user installed packages without removing any base packages for R or MRO.
rmAllPkgs <- function() {

  print("************************** Uninstall All Packages  *****************************")
  # Create a list of all installed packages
  resPkgs <- as.data.frame(installed.packages())
  head(resPkgs)

  # Make sure that no packages in this library will be removed if using MRO
  resPkgs <- subset(resPkgs, !grepl("MRO", resPkgs$LibPath))

  # Ignore base or recommended packages either
  resPkgs <- resPkgs[!(resPkgs[,"Priority"] %in% c("base", "recommended")),]

  # Determine the library where the packages are installed
  path.lib <- unique(resPkgs$LibPath)

  # Create a vector with all the names of the packages you want to remove
  pkgs.to.remove <- resPkgs[,1]
  head(pkgs.to.remove)

  # Remove the packages
  sapply(pkgs.to.remove, remove.packages, lib = path.lib)

}

# Install getopt and revest if not exist in order to specify packages for download and 
depPkgs()

# Get options, using the spec as defined by the enclosed list. It reads the options from the default: commandArgs(TRUE).
library('getopt')
spec = matrix(c(
  'verbose',    'v',   2, "integer",
  'help'   ,    'h',   0, "logical",
  'package',    'p',   1, "character",
  'operation',  'o',   1, "character"
), byrow=TRUE, ncol=4)

opt = getopt(spec)

# If help was asked for print a friendly message and exit with a non-zero error code
if ( !is.null(opt$help) ) {
  # cat(getopt(spec, usage=TRUE))
  # q(status=1)

  write(
    "Usage: ./r-packman.rs -o <install|uninstall> -p <package name>"
    ,
    stderr()
  )

}

# Set some reasonable defaults for the options that are needed, but were not specified.
if ( is.null(opt$verbose   ) )  { opt$verbose    = FALSE }
if ( is.null(opt$help      ) )  { opt$help       = 0     }
if ( is.null(opt$operatio  ) )  { opt$operation  = 0     }
if ( is.null(opt$package   ) )  { opt$package    = 0     }

# Print some progress messages to stderr, if requested.
if ( opt$verbose ) {
  write(
    "
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
    ",
    stderr()
  )
}

if ( !is.null(opt$package) && !is.null(opt$operation) ) {
  if ((tolower(opt$operation) == tolower("download")) && (tolower(opt$package) != tolower("all")))      { getPkgs(opt$package) }
  if ((tolower(opt$operation) == tolower("download")) && (tolower(opt$package) == tolower("all")))      { getAllPkgs() }
  if ((tolower(opt$operation) == tolower("uninstall")) && (tolower(opt$package) == tolower("all")))     { rmAllPkgs() }
  if ((tolower(opt$operation) == tolower("uninstall")) && (tolower(opt$package) != tolower("all")))     { rmPkgs(opt$package) }
  if ( tolower(opt$operation) == tolower("install"))                                                    { setPkgs(opt$package) }
  if ( tolower(opt$operation) == tolower("update"))                                                     { updPkgs(opt$package) }
  if ( tolower(opt$operation) == tolower("delete"))                                                     { delPkgs(opt$package) }
}
