#!/bin/sh

# As modules need to be integrated at different places in the source code, this script
# helps with this integration. It basically add all the references where it's required
# and create a new generic source file so the developper can focus on the core.

if [ -z $1 ]
then
    echo "Missing module name."
    echo "Usage : ./integrate-new-module.sh <module name>"
    exit 1
fi

MODULENAME=$1
RefModuleName=$(echo $1 | sed 's/./\U&/')

# TODO : add following checks on the module name
# - no spaces
# - only alphabetical characters
# - no upper case
# - no special characters
# - no ... ?



# Manually add the module to README.md list
# Manually create an example out of it in examples/

# Create actual module file if it doesn't already exist or if it's empty
prefill_module_file()
{
    MODULENAME=$1
    RefModuleName=$2
    echo "Prefill module file"
    cat template.rs.tmp >> $MODULENAME.rs
    sed -i "s/\*\*ModuleName\*\*/$RefModuleName/g" $MODULENAME.rs 
}

if [ -f $MODULENAME.rs ]
then
    if [ -s $MODULENAME.rs ]
    then
        echo "File exists already"
    else
        prefill_module_file $MODULENAME $RefModuleName
    fi
else
    echo "Create module file for $MODULENAME"
    touch $MODULENAME.rs
    prefill_module_file $MODULENAME $RefModuleName
fi


########### taskexec/src/modules/blocks.rs (2) #######
sed -i "/\*\*BEACON\*\*/a\\
pub use crate::modules::$MODULENAME::$(echo $RefModuleName)ApiCall;" blocks.rs
sed -i "/\*\*BEACON\*\*/a\\
pub use crate::modules::$MODULENAME::$(echo $RefModuleName)BlockExpectedState;" blocks.rs
sed -i "/\*\*BEACON\*\*/a\\
" blocks.rs

########### taskexec/src/modules/mod.rs (4) #######
sed -i "/\*\*BEACON_1\*\*/a\\
pub mod $MODULENAME;" mod.rs
sed -i "/\*\*BEACON_2\*\*/a\\
    $RefModuleName($(echo $RefModuleName)BlockExpectedState)," mod.rs
sed -i "/\*\*BEACON_3\*\*/a\\
            ModuleBlockExpectedState::$RefModuleName(block) => { block.dry_run_block(hosthandler, privilege) }" mod.rs
sed -i "/\*\*BEACON_4\*\*/a\\
    $RefModuleName($(echo $RefModuleName)ApiCall)," mod.rs

########### taskexec/src/workflow/change.rs (2) #######
sed -i "/\*\*BEACON_1\*\*/a\\
                        ModuleApiCall::$RefModuleName(block) => { block.display() }" ../workflow/change.rs
sed -i "/\*\*BEACON_2\*\*/a\\
                        ModuleApiCall::$RefModuleName(block) => { block.apply_moduleblock_change(hosthandler) }" ../workflow/change.rs

########### taskexec/src/workflow/task.rs (2) #######
sed -i "/\*\*BEACON_1\*\*/a\\
    pub $MODULENAME: Option<$(echo $RefModuleName)BlockExpectedState>," ../workflow/task.rs
sed -i "/\*\*BEACON_2\*\*/a\\
        if let Some(content) = self.$MODULENAME.clone() { counter += 1; self.moduleblock = Some(ModuleBlockExpectedState::$RefModuleName(content)); }" ../workflow/task.rs
