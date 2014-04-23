use libc::c_char;

use ffi::*;
use types::*;
use translation_unit::TranslationUnit;

/**
 * Compilation Data
 **/

pub struct CompilationCommand {
    pub cwd: ~str,
    pub args: Vec<~str>
}

impl CompilationCommand {
    pub fn new(compile_command: &CXCompileCommand) -> CompilationCommand {
        let mut args = Vec::new();
        unsafe {
            let num_args = clang_CompileCommand_getNumArgs(*compile_command);
            for idx in range(1, num_args - 4) {
                args.push(clang_CompileCommand_getArg(*compile_command, idx).to_str());
            }
        }
        let cwd = unsafe { clang_CompileCommand_getDirectory(*compile_command) };
        CompilationCommand { args: args, cwd: cwd.to_str() }
    }

    pub fn args_as_c_vec(&self) -> Vec<*c_char> {
        let mut c_args = Vec::new();
        for idx in range(0, self.args.len()) {
            let arg = self.args.get(idx);
            c_args.push(unsafe { arg.to_c_str().unwrap() });
        }
        c_args
    }
}

/**
 * Compilation Database
 **/

pub struct CompilationDatabase {
    cx_c_db : CXCompilationDatabase
}

impl Drop for CompilationDatabase {
    fn drop(&mut self) {
        unsafe { clang_CompilationDatabase_dispose(self.cx_c_db); }
    }
}

impl CompilationDatabase {
    pub fn from_directory(build_dir: &str) -> Result<CompilationDatabase, CXCompilationDatabase_Error> {
        let error = 0;

        let cx_c_db = build_dir.with_c_str(|_build_dir| {
            unsafe { clang_CompilationDatabase_fromDirectory(_build_dir, &error) }
        });

        if error == CXCompilationDatabase_CanNotLoadDatabase as i32 {
            return Err(CXCompilationDatabase_CanNotLoadDatabase);
        }

        Ok(CompilationDatabase { cx_c_db: cx_c_db })
    }

    pub fn compilation_command_for(&self, file_name: &str) -> Option<CompilationCommand> {
        let compile_commands = file_name.with_c_str(|_file_name| {
            unsafe { clang_CompilationDatabase_getCompileCommands(self.cx_c_db, _file_name) }
        });

        let compilation_args = unsafe {
            let num_commands = clang_CompileCommands_getSize(compile_commands);
            if num_commands < 1 {
                clang_CompileCommands_dispose(compile_commands);
                return None;
            }

            let command = clang_CompileCommands_getCommand(compile_commands, 0);
            CompilationCommand::new(&command)
        };

        unsafe { clang_CompileCommands_dispose(compile_commands); }
        Some(compilation_args)
    }

    pub fn translation_unit_for(&self, file_name: &str) -> Option<TranslationUnit> {
        let result = self.compilation_command_for(file_name);
        match result {
            Some(c_data) => Some(TranslationUnit::new(&c_data, file_name)),
            None => None
        }
    }
}
