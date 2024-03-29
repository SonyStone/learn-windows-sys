use windows::Win32::Foundation::{
    GetLastError, SetLastError, ERROR_ACCESS_DENIED, ERROR_ADAP_HDW_ERR, ERROR_ALREADY_ASSIGNED,
    ERROR_ARENA_TRASHED, ERROR_BAD_COMMAND, ERROR_BAD_DEV_TYPE, ERROR_BAD_ENVIRONMENT,
    ERROR_BAD_FORMAT, ERROR_BAD_LENGTH, ERROR_BAD_NETPATH, ERROR_BAD_NET_NAME, ERROR_BAD_NET_RESP,
    ERROR_BAD_REM_ADAP, ERROR_BAD_UNIT, ERROR_CANNOT_MAKE, ERROR_CRC, ERROR_CURRENT_DIRECTORY,
    ERROR_DEV_NOT_EXIST, ERROR_DUP_NAME, ERROR_FAIL_I24, ERROR_FILE_EXISTS, ERROR_FILE_NOT_FOUND,
    ERROR_GEN_FAILURE, ERROR_HANDLE_DISK_FULL, ERROR_HANDLE_EOF, ERROR_INVALID_ACCESS,
    ERROR_INVALID_BLOCK, ERROR_INVALID_DATA, ERROR_INVALID_DRIVE, ERROR_INVALID_FUNCTION,
    ERROR_INVALID_HANDLE, ERROR_INVALID_PARAMETER, ERROR_INVALID_PASSWORD, ERROR_LOCK_VIOLATION,
    ERROR_NETNAME_DELETED, ERROR_NETWORK_ACCESS_DENIED, ERROR_NETWORK_BUSY, ERROR_NET_WRITE_FAULT,
    ERROR_NOT_DOS_DISK, ERROR_NOT_ENOUGH_MEMORY, ERROR_NOT_READY, ERROR_NOT_SAME_DEVICE,
    ERROR_NOT_SUPPORTED, ERROR_NO_MORE_FILES, ERROR_NO_PROC_SLOTS, ERROR_NO_SPOOL_SPACE,
    ERROR_OUTOFMEMORY, ERROR_OUT_OF_PAPER, ERROR_OUT_OF_STRUCTURES, ERROR_PATH_NOT_FOUND,
    ERROR_PRINTQ_FULL, ERROR_PRINT_CANCELLED, ERROR_READ_FAULT, ERROR_REDIR_PAUSED,
    ERROR_REM_NOT_LIST, ERROR_REQ_NOT_ACCEP, ERROR_SECTOR_NOT_FOUND, ERROR_SEEK,
    ERROR_SHARING_BUFFER_EXCEEDED, ERROR_SHARING_PAUSED, ERROR_SHARING_VIOLATION,
    ERROR_TOO_MANY_CMDS, ERROR_TOO_MANY_NAMES, ERROR_TOO_MANY_OPEN_FILES,
    ERROR_TOO_MANY_SEMAPHORES, ERROR_TOO_MANY_SESS, ERROR_UNEXP_NET_ERR, ERROR_WRITE_FAULT,
    ERROR_WRITE_PROTECT, ERROR_WRONG_DISK, NO_ERROR,
};

pub fn last_error() -> String {
    let error = unsafe { GetLastError() };
    unsafe { SetLastError(NO_ERROR) }

    match error {
        NO_ERROR => "NO_ERROR: The operation completed successfully.".to_string(),
        ERROR_INVALID_FUNCTION => "ERROR_INVALID_FUNCTION: Incorrect function.".to_string(),
        ERROR_FILE_NOT_FOUND => "ERROR_FILE_NOT_FOUND: The system cannot find the file specified.".to_string(),
        ERROR_PATH_NOT_FOUND => "ERROR_PATH_NOT_FOUND: The system cannot find the path specified.".to_string(),
        ERROR_TOO_MANY_OPEN_FILES => "ERROR_TOO_MANY_OPEN_FILES: The system cannot open the file.".to_string(),
        ERROR_ACCESS_DENIED => "ERROR_ACCESS_DENIED: Access is denied.".to_string(),
        ERROR_INVALID_HANDLE => "ERROR_INVALID_HANDLE: The handle is invalid.".to_string(),
        ERROR_ARENA_TRASHED => "ERROR_ARENA_TRASHED: The storage control blocks were destroyed.".to_string(),
        ERROR_NOT_ENOUGH_MEMORY => "ERROR_NOT_ENOUGH_MEMORY: Not enough memory resources are available to process this command.".to_string(),
        ERROR_INVALID_BLOCK => "ERROR_INVALID_BLOCK: The storage control block address is invalid.".to_string(),
        ERROR_BAD_ENVIRONMENT => "ERROR_BAD_ENVIRONMENT: The environment is incorrect.".to_string(),
        ERROR_BAD_FORMAT => "ERROR_BAD_FORMAT: An attempt was made to load a program with an incorrect format.".to_string(),
        ERROR_INVALID_ACCESS => "ERROR_INVALID_ACCESS: The access code is invalid.".to_string(),
        ERROR_INVALID_DATA => "ERROR_INVALID_DATA: The data is invalid.".to_string(),
        ERROR_OUTOFMEMORY => "ERROR_OUTOFMEMORY: Not enough storage is available to complete this operation.".to_string(),
        ERROR_INVALID_DRIVE => "ERROR_INVALID_DRIVE: The system cannot find the drive specified.".to_string(),
        ERROR_CURRENT_DIRECTORY => "ERROR_CURRENT_DIRECTORY: The directory cannot be removed.".to_string(),
        ERROR_NOT_SAME_DEVICE => "ERROR_NOT_SAME_DEVICE: The system cannot move the file to a different disk drive.".to_string(),
        ERROR_NO_MORE_FILES => "ERROR_NO_MORE_FILES: There are no more files.".to_string(),
        ERROR_WRITE_PROTECT => "ERROR_WRITE_PROTECT: The media is write protected.".to_string(),
        ERROR_BAD_UNIT => "ERROR_BAD_UNIT: The system cannot find the device specified.".to_string(),
        ERROR_NOT_READY => "ERROR_NOT_READY: The device is not ready.".to_string(),
        ERROR_BAD_COMMAND => "ERROR_BAD_COMMAND: The device does not recognize the command.".to_string(),
        ERROR_CRC => "ERROR_CRC: Data error (cyclic redundancy check).".to_string(),
        ERROR_BAD_LENGTH => "ERROR_BAD_LENGTH: The program issued a command but the command length is incorrect.".to_string(),
        ERROR_SEEK => "ERROR_SEEK: The drive cannot locate a specific area or track on the disk.".to_string(),
        ERROR_NOT_DOS_DISK => "ERROR_NOT_DOS_DISK: The specified disk or diskette cannot be accessed.".to_string(),
        ERROR_SECTOR_NOT_FOUND => "ERROR_SECTOR_NOT_FOUND: The drive cannot find the sector requested.".to_string(),
        ERROR_OUT_OF_PAPER => "ERROR_OUT_OF_PAPER: The printer is out of paper.".to_string(),
        ERROR_WRITE_FAULT => "ERROR_WRITE_FAULT: The system cannot write to the specified device.".to_string(),
        ERROR_READ_FAULT => "ERROR_READ_FAULT: The system cannot read from the specified device.".to_string(),
        ERROR_GEN_FAILURE => "ERROR_GEN_FAILURE: A device attached to the system is not functioning.".to_string(),
        ERROR_SHARING_VIOLATION => "ERROR_SHARING_VIOLATION: The process cannot access the file because it is being used by another process.".to_string(),
        ERROR_LOCK_VIOLATION => "ERROR_LOCK_VIOLATION: The process cannot access the file because another process has locked a portion of the file.".to_string(),
        ERROR_WRONG_DISK => "ERROR_WRONG_DISK: The wrong diskette is in the drive. Insert %2 (Volume Serial Number: %3) into drive %1.".to_string(),
        ERROR_SHARING_BUFFER_EXCEEDED => "ERROR_SHARING_BUFFER_EXCEEDED: Too many files opened for sharing.".to_string(),
        ERROR_HANDLE_EOF => "ERROR_HANDLE_EOF: Reached the end of the file.".to_string(),
        ERROR_HANDLE_DISK_FULL => "ERROR_HANDLE_DISK_FULL: The disk is full.".to_string(),
        ERROR_NOT_SUPPORTED => "ERROR_NOT_SUPPORTED: The request is not supported.".to_string(),
        ERROR_REM_NOT_LIST => "ERROR_REM_NOT_LIST: Windows cannot find the network path. Verify that the network path is correct and the destination computer is not busy or turned off. If Windows still cannot find the network path, contact your network administrator.".to_string(),
        ERROR_DUP_NAME => "ERROR_DUP_NAME: You were not connected because a duplicate name exists on the network. If joining a domain, go to System in Control Panel to change the computer name and try again. If joining a workgroup, choose another workgroup name.".to_string(),
        ERROR_BAD_NETPATH => "ERROR_BAD_NETPATH: The network path was not found.".to_string(),
        ERROR_NETWORK_BUSY => "ERROR_NETWORK_BUSY: The network is busy.".to_string(),
        ERROR_DEV_NOT_EXIST => "ERROR_DEV_NOT_EXIST: The specified network resource or device is no longer available.".to_string(),
        ERROR_TOO_MANY_CMDS => "ERROR_TOO_MANY_CMDS: The network BIOS command limit has been reached.".to_string(),
        ERROR_ADAP_HDW_ERR => "ERROR_ADAP_HDW_ERR: A network adapter hardware error occurred.".to_string(),
        ERROR_BAD_NET_RESP => "ERROR_BAD_NET_RESP: The specified server cannot perform the requested operation.".to_string(),
        ERROR_UNEXP_NET_ERR => "ERROR_UNEXP_NET_ERR: An unexpected network error occurred.".to_string(),
        ERROR_BAD_REM_ADAP => "ERROR_BAD_REM_ADAP: The remote adapter is not compatible.".to_string(),
        ERROR_PRINTQ_FULL => "ERROR_PRINTQ_FULL: The printer queue is full.".to_string(),
        ERROR_NO_SPOOL_SPACE => "ERROR_NO_SPOOL_SPACE: Space to store the file waiting to be printed is not available on the server.".to_string(),
        ERROR_PRINT_CANCELLED => "ERROR_PRINT_CANCELLED: Your file waiting to be printed was deleted.".to_string(),
        ERROR_NETNAME_DELETED => "ERROR_NETNAME_DELETED: The specified network name is no longer available.".to_string(),
        ERROR_NETWORK_ACCESS_DENIED => "ERROR_NETWORK_ACCESS_DENIED: Network access is denied.".to_string(),
        ERROR_BAD_DEV_TYPE => "ERROR_BAD_DEV_TYPE: The network resource type is not correct.".to_string(),
        ERROR_BAD_NET_NAME => "ERROR_BAD_NET_NAME: The network name cannot be found.".to_string(),
        ERROR_TOO_MANY_NAMES => "ERROR_TOO_MANY_NAMES: The name limit for the local computer network adapter card was exceeded.".to_string(),
        ERROR_TOO_MANY_SESS => "ERROR_TOO_MANY_SESS: The network BIOS session limit was exceeded.".to_string(),
        ERROR_SHARING_PAUSED => "ERROR_SHARING_PAUSED: The remote server has been paused or is in the process of being started.".to_string(),
        ERROR_REQ_NOT_ACCEP => "ERROR_REQ_NOT_ACCEP: No more connections can be made to this remote computer at this time because there are already as many connections as the computer can accept.".to_string(),
        ERROR_REDIR_PAUSED => "ERROR_REDIR_PAUSED: The specified printer or disk device has been paused.".to_string(),
        ERROR_FILE_EXISTS => "ERROR_FILE_EXISTS: The file exists.".to_string(),
        ERROR_CANNOT_MAKE => "ERROR_CANNOT_MAKE: The directory or file cannot be created.".to_string(),
        ERROR_FAIL_I24 => "ERROR_FAIL_I24: Fail on INT 24.".to_string(),
        ERROR_OUT_OF_STRUCTURES => "ERROR_OUT_OF_STRUCTURES: Storage to process this request is not available.".to_string(),
        ERROR_ALREADY_ASSIGNED => "ERROR_ALREADY_ASSIGNED: The local device name is already in use.".to_string(),
        ERROR_INVALID_PASSWORD => "ERROR_INVALID_PASSWORD: The specified network password is not correct.".to_string(),
        ERROR_INVALID_PARAMETER => "ERROR_INVALID_PARAMETER: The parameter is incorrect.".to_string(),
        ERROR_NET_WRITE_FAULT => "ERROR_NET_WRITE_FAULT: A write fault occurred on the network.".to_string(),
        ERROR_NO_PROC_SLOTS => "ERROR_NO_PROC_SLOTS: The system cannot start another process at this time.".to_string(),
        ERROR_TOO_MANY_SEMAPHORES => "ERROR_TOO_MANY_SEMAPHORES: Cannot create another system semaphore.".to_string(),
        _ => format!("TODO error {:?}", error),
    }
}
