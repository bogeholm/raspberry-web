#!/usr/bin/env python3

import filecmp # https://stackoverflow.com/questions/254350
import os
import platform # https://stackoverflow.com/questions/1854
import shutil

# Copy following files:

# If there are changes:
# target/release/raspberry-web          -> /usr/local/bin/
# raspberry-web-db/raspberry-web.sqlite -> /usr/local/raspberry-web/database/

# If it does not exist
# config/configuration.toml             -> /usr/local/raspberry-web/

# If on Linux, and if it does not exist
# config/raspberry-web.service          -> /etc/systemd/system/

# Colors for highlighting status
# https://svn.blender.org/svnroot/bf-blender/trunk/blender/build_files/scons/tools/bcolors.py
# https://stackoverflow.com/questions/287871/print-in-terminal-with-colors
okgreen = "\033[1;32;40m"
failred = "\033[1;31;40m"
highlight = "\033[1;34;40m"
endc = "\033[0m"
OK = okgreen + " OK" + endc
ERROR = failred + "ERROR: " + endc

# Path to crate root
crate_root = os.path.dirname(os.getcwd()) + "/"

# Filenames
binary = "raspberry-web"
database = "raspberry-web.sqlite"
config = "configuration.toml"
service = "raspberry-web.service"

# Source paths
binary_source = crate_root + "target/release/"
config_source = crate_root + "config/"
database_soure = crate_root + "raspberry-web-db/"
systemd_source = crate_root + "config/"

# Target paths
binary_target = "/usr/local/bin/"
config_target = "/usr/local/raspberry-web/"
database_target = "/usr/local/raspberry-web/database/"
systemd_target = "/etc/systemd/system/"

def mkdir_if_not_exists(path):
    """ Create directory 'path' if it does not exist
    """
    if not os.path.exists(path):
        os.makedirs(path)
        print("Created directory" + highlight + path + endc + OK)
    else:
        print(highlight + path + endc + " is already present" + OK)


def cp_if_dst_different(source, target):
    """ Copy file from 'source' to 'target' if target file is different than source,
        or if it does not exist. Overwrite if target is different than source.

        Return true if file is copied, false otherwise
    """
    if os.path.isfile(target):
        if not filecmp.cmp(source, target, shallow=False):
            shutil.copy2(source, target)
            print("Added new version of file " + highlight + target + endc + OK)
            return True
        else:
            print(highlight + target + endc + " already present in same version" + OK)
            return False
    else:
        shutil.copy2(source, target)
        print("Added file " + highlight + target + endc + OK)
        return True


def cp_if_dst_nonexistant(source, target):
    """ Copy file from 'source' to 'target' if target file does not exist.

        Return true if file is copied, false otherwise
    """
    if not os.path.isfile(target):
        shutil.copy2(source, target)
        print("Added file " + highlight + target + endc + OK)
        return True
    else:
        print(highlight + target + endc + " already present" + OK)
        return False


def check_file_exists_or_exit(path):
    """ Usecase: check if source files are present - else exit.
    """
    if os.path.isfile(path):
        print("Found source file " + highlight + path + endc + OK)
    else:
        print(ERROR + highlight + path + endc + " not present.")
        exit()


# Supported OS's for this installer
supported_os = ['Linux', 'Darwin']

if __name__ == "__main__":
    # Check if OS is supported
    system = platform.system()
    if system not in supported_os:
        print(ERROR + highlight + system + endc + " is not supported.")
        exit()

    # Check if source files exists - else exit:
    print("Checking source files...")
    soure_files = [
        binary_source+binary,
        config_source+config,
        database_soure+database,
        systemd_source+service]
    for path in soure_files:
        check_file_exists_or_exit(path)
    print()

    # Create config_path and database_path if they do not exist
    print("Checking target directories...")
    for path in [config_target, database_target]:
        mkdir_if_not_exists(path)
    print()

    # Binary
    print("Copying files...")
    binary_copied = cp_if_dst_different(binary_source+binary, binary_target+binary)
    if binary_copied:
        os.chmod(binary_target+binary, 0o755)
    # Configuration
    config_copied = cp_if_dst_nonexistant(config_source+config, config_target+config)
    if config_copied:
        os.chmod(config_target+config, 0o644)
    # Database
    database_copied = cp_if_dst_nonexistant(database_soure+database, database_target+database)
    if database_copied:
        os.chmod(database_target+database, 0o644)
    # Service - only on Linux
    if system == 'Linux':
        service_copied = cp_if_dst_nonexistant(systemd_source+service, systemd_target+service)
        if service_copied:
            os.chmod(systemd_target+service, 0o644)
    else:
        print("Not adding " + highlight + service + endc + " to " + highlight + systemd_target \
            + endc + " since we are running " + highlight + system + endc + OK)
    
    print()
    print(okgreen + "Done." + endc)
