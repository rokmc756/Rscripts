#!/usr/bin/env Rscript
#
#
#
# VMwrae Tanzu Support for Greenplum
# Staff Product Support Engineer
# Jack, Moon <moonja@vmware.com>
#
# ChangeLog
# 2023-05-20 - Cloned initally from http://gpdbkr.blogspot.com/search/label/GPDB6%20R%EC%84%A4%EC%B9%98

mainDir <- "./"
subDir <- "r-pkgs"
dir.create(file.path(mainDir, subDir), showWarnings = FALSE)
# mirrorSite <- "https://cran.yu.ac.kr"  # The main site - "https://cloud.r-project.org"
mirrorSite <- "https://cloud.r-project.org"

depPackages <- function() {
  dep_packages <- c( "getopt" )                                        		         # Specify your packages
  not_installed <- dep_packages[!(dep_packages %in% installed.packages()[ , "Package"])] # Extract not installed packages
  options(repos = c(CRAN <- mirrorSite))
  if(length(not_installed)) install.packages(not_installed)
}

#
infoPackages <- function(packs) {
  packages <- unlist(
    # Find (recursively) dependencies or reverse dependencies of packages.
    tools::package_dependencies(packs, available.packages(), which=c("Depends", "Imports"), recursive=TRUE)
  )
  packages <- union(packs, packages)
  return(packages)
}

#
getPackages <- function(packs) {
  options(repos = c(CRAN <- mirrorSite))
  packages <- infoPackages(c(packs))
  download.packages(packages, destdir=file.path(mainDir, subDir), type="binary")
}

#
setPackages <- function(packs) {
  pkg.list <- list.files( file.path(mainDir, subDir) )
  pkg.list <- setdiff( pkg.list, installed.packages()[, "Package"] )
  for( p in pkg.list ) {
    pkg.path <- file.path( file.path(mainDir, subDir), p )
    install.packages( pkg.path, repos = NULL, dependencies = TRUE, tpye=binary )
    # install.packages(pk.path, repos = NULL, type="source")
  }
}

rmPackages <- function(packs) {
  #file.list <- list.files( file.path(mainDir, subDir) )
  #pkg.list <- setdiff( file.list, installed.packages()[, "Package"] )
  #for( p in pkg.list ) {
  #  pkg.path <- file.path( file.path(mainDir, subDir), p )
  remove.packages( packs )
    #, repos = NULL, dependencies = TRUE )
    # install.packages(pk.path, repos = NULL, type="source")
  #}
}

rmAllPackages <- function() {

  # Here is the script I’ve created in order to remove all user installed packages, without removing any base packages for R or MRO.
  # Create a list of all installed packages
  resPkgs <- as.data.frame(installed.packages())
  head(resPkgs)

  # Make sure that no packages in this library will be removed if using MRO
  resPkgs <- subset(resPkgs, !grepl("MRO", resPkgs$LibPath))

  # Don't remove base or recommended packages either\
  resPkgs <- resPkgs[!(resPkgs[,"Priority"] %in% c("base", "recommended")),]

  # Determine the library where the packages are installed
  path.lib <- unique(resPkgs$LibPath)

  # Create a vector with all the names of the packages you want to remove
  pkgs.to.remove <- resPkgs[,1]
  head(pkgs.to.remove)

  # Remove the packages
  sapply(pkgs.to.remove, remove.packages, lib = path.lib)

}



# Install getopt if not exist in order to specify packages for download
depPackages()

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
  cat(getopt(spec, usage=TRUE))
  q(status=1)
}

# Set some reasonable defaults for the options that are needed, but were not specified.
if ( is.null(opt$verbose   ) )  { opt$verbose    = FALSE }
if ( is.null(opt$help      ) )  { opt$help       = 0     }
if ( is.null(opt$operation  ) ) { opt$operation  = 0     }
if ( is.null(opt$package   ) )  { opt$package    = 0     }

# Print some progress messages to stderr, if requested.
if ( opt$verbose ) { write("Writing...",stderr()) }

if ( !is.null(opt$package) && !is.null(opt$operation) ) {
	if ( tolower(opt$operation) == tolower("get"))                                                   { getPackages(opt$package) }
	if ( tolower(opt$operation) == tolower("setup"))                                                 { setPackages(opt$package) }
	if ( tolower(opt$operation) == tolower("delete"))                                                { delPackages(opt$package) }
	if ((tolower(opt$operation) == tolower("remove")) && (tolower(opt$package) != tolower("all")))   { rmPackages(opt$package) }
	if ((tolower(opt$operation) == tolower("remove")) && (tolower(opt$package) == tolower("all")))   { rmAllPackages() }

}

# Do some operation based on user input.
# cat(paste(rnorm(opt$count,mean=opt$mean,sd=opt$sd),collapse="\n"))
# cat("\n")

# signal success and exit.
# q(status=0)

# Install R binardy 
# https://docs.posit.co/resources/install-r/
# https://docs.posit.co/rspm/admin/repositories/#multiple-sources
# ttps://docs.posit.co/rspm/admin/appendix/system-dependency-detection/
# https://github.com/rstudio/r-builds
# https://www.rdocumentation.org/packages/utils/versions/3.6.2/topics/install.packages
# https://cran.r-project.org/bin/windows/base/howto-R-devel.html
# https://cran.r-project.org/doc/manuals/r-release/R-admin.html
