//program API, (de)serializing instruction data

//inside instruction.rs

// add InitEscrow API endpoint
pub enum EscrowInstruction {
        /// Starts the trade by creating and populating an escrow account and transferring
        ///  ownership of the given temp token account to the PDA

        /// Accounts expected:
        /// 
        /// 
        /// 0. `[signer]` The account of token person initizalizing the escrow
        /// 1. `[writable]` Temporary token account that should be created prior 
        /// to this instruction and owned by the initializer
        /// 2. `[]` The initializer's token account for the token they will receive 
        /// should the trade go through
        /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade => Prallelisation property
        /// 4. `[]` The rent sysvar
        /// 5. `[]` The token program
        /// 
        ///
        /// 
        /// 
        ///  0. `[signer]: Tx process: Alice -> Bob (Alice send tx to Bob), and thus ALice as the initializer, and Bob as the taker
        /// So we need Account - and specifically Account 0 as a signer because transferring the ownership of the 
        /// temporary account requires Alice's signature

        ///  1. `[writable]`:Account 1 is the temp token X account which needs to be writable. This is because changing token account 
        /// ownership is a user space change which means the data field of the account will be changed     

        /// 2. `[]`  Account 2 is Alice's token Y account. While it will be written to eventually, it won't happen in this transaction which is
        ///  why we can leave the brackets empty (implying read-only)
        /// 
        /// 3. `[writable]`, Account 3 is the escrow account which also needs to be writable because the program will 
        /// write the escrow information into it
        /// 
        /// 4. `[],Account 4 is the Rent sysvar. I'll explain this in detail once we get to writing the processor code.
        /// 
        /// 5. `[]` The token program:  Account 5 is the account of the token program itself. I will explain why we need this account as well 
        /// when we get to writing the processor code.


InitEscrow {    
    // The amount part A expect to receive of token Y
    amount: u64
}

}