#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]

mod api;
use api::*;

use num_traits::*;
use xous_ipc::Buffer;
use xous::{msg_blocking_scalar_unpack, msg_scalar_unpack};

use core::sync::atomic::{AtomicBool, Ordering};
#[cfg(any(target_os = "none", target_os = "xous"))]
mod implementation {
    use utralib::generated::*;
    use crate::api::*;
    use susres::{RegManager, RegOrField, SuspendResume};
    use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
    use num_traits::*;

    #[derive(Debug)]
    enum FlashOp {
        EraseSector(u32), // 4k sector
        WritePages(u32, [u8; 4096], usize), // page address, data, len
        ReadId,
    }

    static SPINOR_RUNNING: AtomicBool = AtomicBool::new(false);
    static SPINOR_RESULT: AtomicU32 = AtomicU32::new(0);
    fn spinor_safe_context(_irq_no: usize, arg: *mut usize) {
        let spinor = unsafe { &mut *(arg as *mut Spinor) };

        let mut result = 0;
        match spinor.cur_op {
            Some(FlashOp::EraseSector(sector_address)) => {
                // enable writes: set wren mode
                loop {
                    flash_wren(&mut spinor.csr);
                    let status = flash_rdsr(&mut spinor.csr, 1);
                    if status & 0x02 != 0 {
                        break;
                    }
                }
                // issue erase command
                flash_se4b(&mut spinor.csr, sector_address);
                // wait for WIP bit to drop
                loop {
                    let status = flash_rdsr(&mut spinor.csr, 1);
                    if status & 0x01 == 0 {
                        break;
                    }
                }
                // get the success code for return
                result = flash_rdscur(&mut spinor.csr);
                // disable writes: send wrdi
                if flash_rdsr(&mut spinor.csr, 1) & 0x02 != 0 {
                    loop {
                        flash_wrdi(&mut spinor.csr);
                        let status = flash_rdsr(&mut spinor.csr, 1);
                        if status & 0x02 == 0 {
                            break;
                        }
                    }
                }
                flash_rdsr(&mut spinor.csr, 0); // dummy read to clear the "read lock" bit
            },
            Some(FlashOp::WritePages(start_addr, data, len)) => {
                // assumption: data being written to is already erased (set to 0xFF)
                assert!(len <= 4096, "data len is too large");
                assert!((len % 2) == 0, "data is not a multiple of 2 in length: the SPI DDR interface always requires two bytes per transfer");
                let mut cur_addr = start_addr;
                let mut pre_align = 0;
                let mut more_aligned_pages = true;
                if cur_addr & 0xff != 0 {
                    // do a partial-page program to get us into page alignment:
                    //   - it's OK to send an address that isn't page-aligned, but:
                    //   - you can only write data that would program up to the end of the page
                    //   - excess data would "wrap around" and program bytes at the beginning of the page, which is incorrect behavior
                    pre_align = 0x100 - (cur_addr & 0xFF);

                    if pre_align >= len as u32 {
                        pre_align = len as u32;
                        more_aligned_pages = false;
                    }

                    let partial_page = &data[0..pre_align as usize];
                    // check for blank writes and skip
                    let mut blank = true;
                    for word in partial_page.chunks(2) {
                        // if the data is blank, don't do a write
                        let wdata = word[0] as u32 | ((word[1] as u32) << 8);
                        if wdata != 0xFFFF {
                            blank = false; // short circuit evaluation if we find anything that's not blank
                            break;
                        }
                    }
                    if !blank {
                        // enable writes: set wren mode
                        loop {
                            flash_wren(&mut spinor.csr);
                            let status = flash_rdsr(&mut spinor.csr, 1);
                            if status & 0x02 != 0 {
                                break;
                            }
                        }
                        // fill the page fifo
                        for word in partial_page.chunks(2) {
                            let wdata = word[0] as u32 | ((word[1] as u32) << 8);
                            spinor.csr.wfo(utra::spinor::WDATA_WDATA, wdata);
                        }
                        // send the data to be programmed
                        flash_pp4b(&mut spinor.csr, cur_addr, partial_page.len() as u32);
                        while (flash_rdsr(&mut spinor.csr, 1) & 0x01) != 0 {
                            // wait while WIP is set
                        }
                        // get the success code for return
                        result = flash_rdscur(&mut spinor.csr);
                    }
                    cur_addr += pre_align; // increment the address, even if we "skipped" the region
                }
                if ((result & 0x20) == 0) && more_aligned_pages {
                    assert!(cur_addr & 0xff == 0, "data is not page-aligned going into the aligned write phase");
                    // now write the remaining, aligned pages. The last chunk can be short of data,
                    // that's also fine; the write will not affect bytes that are not transmitted
                    for page in data[pre_align as usize..len].chunks(256) {
                        // check & skip writes that are blank
                        let mut blank = true;
                        for word in page.chunks(2) {
                            let wdata = word[0] as u32 | ((word[1] as u32) << 8);
                            if wdata != 0xFFFF {
                                blank = false;
                                break;
                            }
                        }
                        if blank {
                            // skip over pages that are entirely blank
                            continue;
                        }
                        // enable writes: set wren mode
                        loop {
                            flash_wren(&mut spinor.csr);
                            let status = flash_rdsr(&mut spinor.csr, 1);
                            if status & 0x02 != 0 {
                                break;
                            }
                        }
                        // fill the fifo
                        for word in page.chunks(2) {
                            let wdata = word[0] as u32 | ((word[1] as u32) << 8);
                            spinor.csr.wfo(utra::spinor::WDATA_WDATA, wdata);
                        }
                        // send the data to be programmed
                        flash_pp4b(&mut spinor.csr, cur_addr, page.len() as u32);
                        cur_addr += page.len() as u32;

                        while (flash_rdsr(&mut spinor.csr, 1) & 0x01) != 0 {
                            // wait while WIP is set
                        }
                        // get the success code for return
                        result = flash_rdscur(&mut spinor.csr);
                        if result & 0x20 != 0 {
                            break; // abort if error
                        }
                    }
                }
                // disable writes: send wrdi
                if flash_rdsr(&mut spinor.csr, 1) & 0x02 != 0 {
                    loop {
                        flash_wrdi(&mut spinor.csr);
                        let status = flash_rdsr(&mut spinor.csr, 1);
                        if status & 0x02 == 0 {
                            break;
                        }
                    }
                }
                flash_rdsr(&mut spinor.csr, 0); // dummy read to clear the "read lock" bit
            },
            Some(FlashOp::ReadId) => {
                let upper = flash_rdid(&mut spinor.csr, 2);
                let lower = flash_rdid(&mut spinor.csr, 1);
                // re-assemble the ID word from the duplicated bytes read
                result = (lower & 0xFF) | ((lower >> 8) & 0xFF00) | (upper & 0xFF_0000);
            },
            None => {
                panic!("Improper entry to SPINOR safe context.");
            }
        }

        spinor.cur_op = None;
        SPINOR_RESULT.store(result, Ordering::Relaxed);
        spinor.softirq.wfo(utra::spinor_soft_int::EV_PENDING_SPINOR_INT, 1);
        SPINOR_RUNNING.store(false, Ordering::Relaxed);
    }

    fn ecc_handler(_irq_no: usize, arg: *mut usize) {
        let spinor = unsafe { &mut *(arg as *mut Spinor) };

        xous::try_send_message(spinor.handler_conn,
            xous::Message::new_scalar(Opcode::EccError.to_usize().unwrap(),
                spinor.csr.rf(utra::spinor::ECC_ADDRESS_ECC_ADDRESS) as usize,
                spinor.csr.rf(utra::spinor::ECC_STATUS_ECC_OVERFLOW) as usize,
                0, 0)).map(|_|()).unwrap();

        spinor.csr.wfo(utra::spinor::EV_PENDING_ECC_ERROR, 1);
    }

    fn flash_rdsr(csr: &mut utralib::CSR<u32>, lock_reads: u32) -> u32 {
        csr.wfo(utra::spinor::CMD_ARG_CMD_ARG, 0);
        csr.wo(utra::spinor::COMMAND,
            csr.ms(utra::spinor::COMMAND_EXEC_CMD, 1)
            | csr.ms(utra::spinor::COMMAND_LOCK_READS, lock_reads)
            | csr.ms(utra::spinor::COMMAND_CMD_CODE, 0x05) // RDSR
            | csr.ms(utra::spinor::COMMAND_DUMMY_CYCLES, 4)
            | csr.ms(utra::spinor::COMMAND_DATA_WORDS, 1)
            | csr.ms(utra::spinor::COMMAND_HAS_ARG, 1)
        );
        while csr.rf(utra::spinor::STATUS_WIP) != 0 {}
        csr.r(utra::spinor::CMD_RBK_DATA)
    }

    fn flash_rdscur(csr: &mut utralib::CSR<u32>) -> u32 {
        csr.wfo(utra::spinor::CMD_ARG_CMD_ARG, 0);
        csr.wo(utra::spinor::COMMAND,
              csr.ms(utra::spinor::COMMAND_EXEC_CMD, 1)
            | csr.ms(utra::spinor::COMMAND_LOCK_READS, 1)
            | csr.ms(utra::spinor::COMMAND_CMD_CODE, 0x2B) // RDSCUR
            | csr.ms(utra::spinor::COMMAND_DUMMY_CYCLES, 4)
            | csr.ms(utra::spinor::COMMAND_DATA_WORDS, 1)
            | csr.ms(utra::spinor::COMMAND_HAS_ARG, 1)
        );
        while csr.rf(utra::spinor::STATUS_WIP) != 0 {}
        csr.r(utra::spinor::CMD_RBK_DATA)
    }

    fn flash_rdid(csr: &mut utralib::CSR<u32>, offset: u32) -> u32 {
        csr.wfo(utra::spinor::CMD_ARG_CMD_ARG, 0);
        csr.wo(utra::spinor::COMMAND,
            csr.ms(utra::spinor::COMMAND_EXEC_CMD, 1)
          | csr.ms(utra::spinor::COMMAND_CMD_CODE, 0x9f)  // RDID
          | csr.ms(utra::spinor::COMMAND_DUMMY_CYCLES, 4)
          | csr.ms(utra::spinor::COMMAND_DATA_WORDS, offset) // 2 -> 0x3b3b8080, // 1 -> 0x8080c2c2
          | csr.ms(utra::spinor::COMMAND_HAS_ARG, 1)
        );
        while csr.rf(utra::spinor::STATUS_WIP) != 0 {}
        csr.r(utra::spinor::CMD_RBK_DATA)
    }

    fn flash_wren(csr: &mut utralib::CSR<u32>) {
        csr.wfo(utra::spinor::CMD_ARG_CMD_ARG, 0);
        csr.wo(utra::spinor::COMMAND,
            csr.ms(utra::spinor::COMMAND_EXEC_CMD, 1)
          | csr.ms(utra::spinor::COMMAND_CMD_CODE, 0x06)  // WREN
          | csr.ms(utra::spinor::COMMAND_LOCK_READS, 1)
        );
        while csr.rf(utra::spinor::STATUS_WIP) != 0 {}
    }

    fn flash_wrdi(csr: &mut utralib::CSR<u32>) {
        csr.wfo(utra::spinor::CMD_ARG_CMD_ARG, 0);
        csr.wo(utra::spinor::COMMAND,
            csr.ms(utra::spinor::COMMAND_EXEC_CMD, 1)
          | csr.ms(utra::spinor::COMMAND_CMD_CODE, 0x04)  // WRDI
          | csr.ms(utra::spinor::COMMAND_LOCK_READS, 1)
        );
        while csr.rf(utra::spinor::STATUS_WIP) != 0 {}
    }

    fn flash_se4b(csr: &mut utralib::CSR<u32>, sector_address: u32) {
        csr.wfo(utra::spinor::CMD_ARG_CMD_ARG, sector_address);
        csr.wo(utra::spinor::COMMAND,
            csr.ms(utra::spinor::COMMAND_EXEC_CMD, 1)
          | csr.ms(utra::spinor::COMMAND_CMD_CODE, 0x21)  // SE4B
          | csr.ms(utra::spinor::COMMAND_HAS_ARG, 1)
          | csr.ms(utra::spinor::COMMAND_LOCK_READS, 1)
        );
        while csr.rf(utra::spinor::STATUS_WIP) != 0 {}
    }

    #[allow(dead_code)]
    fn flash_be4b(csr: &mut utralib::CSR<u32>, block_address: u32) {
        csr.wfo(utra::spinor::CMD_ARG_CMD_ARG, block_address);
        csr.wo(utra::spinor::COMMAND,
            csr.ms(utra::spinor::COMMAND_EXEC_CMD, 1)
          | csr.ms(utra::spinor::COMMAND_CMD_CODE, 0xdc)  // BE4B
          | csr.ms(utra::spinor::COMMAND_HAS_ARG, 1)
          | csr.ms(utra::spinor::COMMAND_LOCK_READS, 1)
        );
        while csr.rf(utra::spinor::STATUS_WIP) != 0 {}
    }

    fn flash_pp4b(csr: &mut utralib::CSR<u32>, address: u32, data_bytes: u32) {
        let data_words = data_bytes / 2;
        assert!(data_words <= 128, "data_words specified is longer than one page!");
        csr.wfo(utra::spinor::CMD_ARG_CMD_ARG, address);
        csr.wo(utra::spinor::COMMAND,
            csr.ms(utra::spinor::COMMAND_EXEC_CMD, 1)
          | csr.ms(utra::spinor::COMMAND_CMD_CODE, 0x12)  // PP4B
          | csr.ms(utra::spinor::COMMAND_HAS_ARG, 1)
          | csr.ms(utra::spinor::COMMAND_DATA_WORDS, data_words)
          | csr.ms(utra::spinor::COMMAND_LOCK_READS, 1)
        );
        while csr.rf(utra::spinor::STATUS_WIP) != 0 {}
    }

    pub struct Spinor {
        id: u32,
        handler_conn: xous::CID,
        csr: utralib::CSR<u32>,
        susres: RegManager::<{utra::spinor::SPINOR_NUMREGS}>,
        softirq: utralib::CSR<u32>,
        cur_op: Option<FlashOp>,
        ticktimer: ticktimer_server::Ticktimer,
        // TODO: refactor ecup command to use spinor to operate the reads
    }

    impl Spinor {
        pub fn new(handler_conn: xous::CID) -> Spinor {
            let csr = xous::syscall::map_memory(
                xous::MemoryAddress::new(utra::spinor::HW_SPINOR_BASE),
                None,
                4096,
                xous::MemoryFlags::R | xous::MemoryFlags::W,
            )
            .expect("couldn't map SPINOR CSR range");
            let softirq = xous::syscall::map_memory(
                xous::MemoryAddress::new(utra::spinor_soft_int::HW_SPINOR_SOFT_INT_BASE),
                None,
                4096,
                xous::MemoryFlags::R | xous::MemoryFlags::W,
            )
            .expect("couldn't map SPINOR soft interrupt CSR range");

            let mut spinor = Spinor {
                id: 0,
                handler_conn,
                csr: CSR::new(csr.as_mut_ptr() as *mut u32),
                softirq: CSR::new(softirq.as_mut_ptr() as *mut u32),
                susres: RegManager::new(csr.as_mut_ptr() as *mut u32),
                cur_op: None,
                ticktimer: ticktimer_server::Ticktimer::new().unwrap(),
            };

            xous::claim_interrupt(
                utra::spinor_soft_int::SPINOR_SOFT_INT_IRQ,
                spinor_safe_context,
                (&mut spinor) as *mut Spinor as *mut usize,
            )
            .expect("couldn't claim SPINOR irq");
            spinor.softirq.wfo(utra::spinor_soft_int::EV_PENDING_SPINOR_INT, 1);
            spinor.softirq.wfo(utra::spinor_soft_int::EV_ENABLE_SPINOR_INT, 1);

            xous::claim_interrupt(
                utra::spinor::SPINOR_IRQ,
                ecc_handler,
                (&mut spinor) as *mut Spinor as *mut usize,
            )
            .expect("couldn't claim SPINOR irq");
            spinor.softirq.wfo(utra::spinor::EV_PENDING_ECC_ERROR, 1);
            spinor.softirq.wfo(utra::spinor::EV_ENABLE_ECC_ERROR, 1);
            spinor.susres.push_fixed_value(RegOrField::Reg(utra::spinor::EV_PENDING), 0xFFFF_FFFF);
            spinor.susres.push(RegOrField::Reg(utra::spinor::EV_ENABLE), None);

            // now populate the id field
            spinor.cur_op = Some(FlashOp::ReadId);
            SPINOR_RUNNING.store(true, Ordering::Relaxed);
            spinor.softirq.wfo(utra::spinor_soft_int::SOFTINT_SOFTINT, 1);
            while SPINOR_RUNNING.load(Ordering::Relaxed) {}
            spinor.id = SPINOR_RESULT.load(Ordering::Relaxed);

            spinor
        }

        /// changes into the spinor interrupt handler context, which is "safe" for ROM operations because we guarantee
        /// we don't touch the SPINOR block inside that interrupt context
        /// we name it with _blocking suffix to remind ourselves that this op should full-block Xous, no exceptions, until the flash op is done.
        fn call_spinor_context_blocking(&mut self) -> u32 {
            if self.cur_op.is_none() {
                log::error!("called with no spinor op set. This is an internal error...panicing!");
                panic!("called with no spinor op set.");
            }
            self.ticktimer.ping_wdt();
            SPINOR_RUNNING.store(true, Ordering::Relaxed);
            self.softirq.wfo(utra::spinor_soft_int::SOFTINT_SOFTINT, 1);
            while SPINOR_RUNNING.load(Ordering::Relaxed) {
                // there is no timeout condition that makes sense. If we're in a very long flash op -- and they can take hundreds of ms --
                // simply timing out and trying to move on could lead to hardware damage as we'd be accessing a ROM that is in progress.
                // in other words: if the flash memory is broke, you're broke too, ain't nobody got time for that.
            }
            self.ticktimer.ping_wdt();
            xous::arch::cache_flush(); // flush the CPU caches in case the SPINOR call modified memory that's cached
            SPINOR_RESULT.load(Ordering::Relaxed)
        }

        pub(crate) fn write_region(&mut self, wr: &mut WriteRegion) -> SpinorError {
            // log::trace!("processing write_region with {:x?}", wr);
            if wr.start + wr.len > SPINOR_SIZE_BYTES { // basic security check. this is necessary so we don't have wrap-around attacks on the SoC gateware region
                return SpinorError::InvalidRequest;
            }

            if !wr.clean_patch {
                // the `lib.rs` side has ostensibly already done the following checks for us:
                //   - made sure there is some "dirty" data in this page
                //   - provided us with a copy of any existing data we want to replace after the patch within the given page
                //   - ensured that the request is aligned with an erase sector
                let alignment_mask = SPINOR_ERASE_SIZE - 1;
                if (wr.start & alignment_mask) != 0 {
                    return SpinorError::AlignmentError;
                }
                self.cur_op = Some(FlashOp::EraseSector(wr.start));
                log::trace!("erase: {:x?}", self.cur_op);
                let erase_result = self.call_spinor_context_blocking();
                if erase_result & 0x40 != 0 {
                    log::error!("E_FAIL set, erase failed: result 0x{:02x}, sector addr 0x{:08x}", erase_result, wr.start);
                    return SpinorError::EraseFailed;
                }

                // now write the data sector
                self.cur_op = Some(FlashOp::WritePages(wr.start, wr.data, wr.len as usize));
                log::trace!("write: len:{}, {:x?}", wr.len, self.cur_op);
                let write_result = self.call_spinor_context_blocking();
                if write_result & 0x20 != 0 {
                    log::error!("P_FAIL set, program failed/partial abort: result 0x{:02x}, sector addr 0x{:08x}", write_result, wr.start);
                    return SpinorError::WriteFailed;
                }
                SpinorError::NoError
            } else {
                // clean patch path:
                // we're just patching sectors -- the caller PROMISES the data has been erased already
                // this function has no way of knowing, because we don't have read priveledges...
                // here, almost any data alignment and length is acceptable -- we can patch even just two bytes using
                // this call.
                self.cur_op = Some(FlashOp::WritePages(wr.start, wr.data, wr.len as usize));
                log::trace!("clean write: len:{}, {:x?}", wr.len, self.cur_op);
                let write_result = self.call_spinor_context_blocking();
                if write_result & 0x20 != 0 {
                    log::error!("P_FAIL set, program failed/partial abort: result 0x{:02x}, sector addr 0x{:08x}", write_result, wr.start);
                    return SpinorError::WriteFailed;
                }
                SpinorError::NoError
            }
        }

        pub fn suspend(&mut self) {
            self.susres.suspend();
        }
        pub fn resume(&mut self) {
            self.susres.resume();
            self.softirq.wfo(utra::spinor_soft_int::EV_PENDING_SPINOR_INT, 1);
            self.softirq.wfo(utra::spinor_soft_int::EV_ENABLE_SPINOR_INT, 1);
        }
    }
}

// a stub to try to avoid breaking hosted mode for as long as possible.
#[cfg(not(any(target_os = "none", target_os = "xous")))]
mod implementation {
    use crate::api::*;
    pub struct Spinor {
    }

    impl Spinor {
        pub fn new(_conn: xous::CID) -> Spinor {
            Spinor {
            }
        }
        pub fn suspend(&self) {
        }
        pub fn resume(&self) {
        }
        pub(crate) fn write_region(&mut self, _wr: &mut WriteRegion) -> SpinorError {
            SpinorError::ImplementationError
        }
    }
}


static OP_IN_PROGRESS: AtomicBool = AtomicBool::new(false);
static SUSPEND_FAILURE: AtomicBool = AtomicBool::new(false);
static SUSPEND_PENDING: AtomicBool = AtomicBool::new(false);

fn susres_thread(sid0: usize, sid1: usize, sid2: usize, sid3: usize) {
    let susres_sid = xous::SID::from_u32(sid0 as u32, sid1 as u32, sid2 as u32, sid3 as u32);
    let xns = xous_names::XousNames::new().unwrap();

    // register a suspend/resume listener
    let sr_cid = xous::connect(susres_sid).expect("couldn't create suspend callback connection");
    let mut susres = susres::Susres::new(&xns, api::SusResOps::SuspendResume as u32, sr_cid).expect("couldn't create suspend/resume object");

    let main_cid = xns.request_connection_blocking(api::SERVER_NAME_SPINOR).expect("couldn't connect to our main thread for susres coordination");

    log::trace!("starting SPINOR suspend/resume manager loop");
    loop {
        let msg = xous::receive_message(susres_sid).unwrap();
        log::trace!("Message: {:?}", msg);
        match FromPrimitive::from_usize(msg.body.id()) {
            Some(SusResOps::SuspendResume) => xous::msg_scalar_unpack!(msg, token, _, _, _, {
                SUSPEND_PENDING.store(true, Ordering::Relaxed);
                while OP_IN_PROGRESS.load(Ordering::Relaxed) {
                    xous::yield_slice();
                }
                xous::send_message(main_cid,
                    xous::Message::new_blocking_scalar(Opcode::SuspendInner.to_usize().unwrap(), 0, 0, 0, 0)).expect("couldn't send suspend message");
                if susres.suspend_until_resume(token).expect("couldn't execute suspend/resume") == false {
                    SUSPEND_FAILURE.store(true, Ordering::Relaxed);
                } else {
                    SUSPEND_FAILURE.store(false, Ordering::Relaxed);
                }
                xous::send_message(main_cid,
                    xous::Message::new_blocking_scalar(Opcode::ResumeInner.to_usize().unwrap(), 0, 0, 0, 0)).expect("couldn't send suspend message");
                SUSPEND_PENDING.store(false, Ordering::Relaxed);
            }),
            Some(SusResOps::Quit) => {
                log::info!("Received quit opcode, exiting!");
                break;
            }
            None => {
                log::error!("Received unknown opcode: {:?}", msg);
            }
        }
    }
    xns.unregister_server(susres_sid).unwrap();
    xous::destroy_server(susres_sid).unwrap();
}


#[xous::xous_main]
fn xmain() -> ! {
    use crate::implementation::Spinor;

    log_server::init_wait().unwrap();
    log::set_max_level(log::LevelFilter::Info);
    log::info!("my PID is {}", xous::process::id());

    let xns = xous_names::XousNames::new().unwrap();
    /*
        Very important to track who has access to the SPINOR server, and limit access. Access to this server is essential for persistent rootkits.
        Here is the list of servers allowed to access, and why:
          - shellchat (for testing ONLY, remove once done)
          - suspend/resume (for suspend locking/unlocking calls)
          - keystore
          - PDDB (not yet written)
    */
    let spinor_sid = xns.register_name(api::SERVER_NAME_SPINOR, Some(3)).expect("can't register server");
    log::trace!("registered with NS -- {:?}", spinor_sid);

    let handler_conn = xous::connect(spinor_sid).expect("couldn't create interrupt handler callback connection");
    let mut spinor = Spinor::new(handler_conn);

    log::trace!("ready to accept requests");

    // handle suspend/resume with a separate thread, which monitors our in-progress state
    // we can't interrupt an erase or program operation, so the op MUST finish before we can suspend.
    let susres_mgr_sid = xous::create_server().unwrap();
    let (sid0, sid1, sid2, sid3) = susres_mgr_sid.to_u32();
    xous::create_thread_4(susres_thread, sid0 as usize, sid1 as usize, sid2 as usize, sid3 as usize).expect("couldn't start susres handler thread");

    let llio = llio::Llio::new(&xns).expect("couldn't connect to LLIO");

    let mut client_id: Option<[u32; 4]> = None;
    let mut soc_token: Option<[u32; 4]> = None;
    let mut ecc_errors: [Option<u32>; 4] = [None, None, None, None]; // just record the first few errors, until we can get `std` and a convenient Vec/Queue
    let mut staging_write_protect: bool = false;

    loop {
        let mut msg = xous::receive_message(spinor_sid).unwrap();
        match FromPrimitive::from_usize(msg.body.id()) {
            Some(Opcode::SuspendInner) => msg_blocking_scalar_unpack!(msg, _, _, _, _, {
                spinor.suspend();
                xous::return_scalar(msg.sender, 1).unwrap();
            }),
            Some(Opcode::ResumeInner) => msg_blocking_scalar_unpack!(msg, _, _, _, _, {
                spinor.resume();
                xous::return_scalar(msg.sender, 1).unwrap();
            }),
            Some(Opcode::RegisterSocToken) => msg_scalar_unpack!(msg, id0, id1, id2, id3, {
                // only the first process to claim it can have it!
                // make sure to do it correctly at boot: this must be done extremely early in the
                // boot process; any attempt to access this unit for functional ops before this is registered shall panic
                // this is to mitigate a DoS attack on the legitimate registrar that gives way for the incorrect
                // process to grab this token
                if soc_token.is_none() {
                    soc_token = Some([id0 as u32, id1 as u32, id2 as u32, id3 as u32]);
                }
            }),
            Some(Opcode::SetStagingWriteProtect)  => msg_scalar_unpack!(msg, id0, id1, id2, id3, {
                if let Some(token) = soc_token {
                    if token[0] == id0 as u32 && token[1] == id1 as u32 &&
                    token[2] == id2 as u32 && token[3] == id3 as u32 {
                        staging_write_protect = true;
                    }
                }
            }),
            Some(Opcode::ClearStagingWriteProtect)  => msg_scalar_unpack!(msg, id0, id1, id2, id3, {
                if let Some(token) = soc_token {
                    if token[0] == id0 as u32 && token[1] == id1 as u32 &&
                    token[2] == id2 as u32 && token[3] == id3 as u32 {
                        staging_write_protect = false;
                    }
                }
            }),
            Some(Opcode::AcquireExclusive) => msg_blocking_scalar_unpack!(msg, id0, id1, id2, id3, {
                if soc_token.is_none() { // reject any ops until a soc token is registered
                    xous::return_scalar(msg.sender, 0).unwrap();
                }
                if client_id.is_none() && !SUSPEND_PENDING.load(Ordering::Relaxed) {
                    OP_IN_PROGRESS.store(true, Ordering::Relaxed); // lock out suspends when the exclusive lock is acquired
                    llio.wfi_override(true).expect("couldn't shut off WFI");
                    client_id = Some([id0 as u32, id1 as u32, id2 as u32, id3 as u32]);
                    log::trace!("giving {:x?} an exclusive lock", client_id);
                    SUSPEND_FAILURE.store(false, Ordering::Relaxed);
                    xous::return_scalar(msg.sender, 1).unwrap();
                } else {
                    xous::return_scalar(msg.sender, 0).unwrap();
                }
            }),
            Some(Opcode::ReleaseExclusive) => msg_blocking_scalar_unpack!(msg, _, _, _, _, {
                client_id = None;
                OP_IN_PROGRESS.store(false, Ordering::Relaxed);
                llio.wfi_override(false).expect("couldn't restore WFI");
                xous::return_scalar(msg.sender, 1).unwrap();
            }),
            Some(Opcode::AcquireSuspendLock) => msg_blocking_scalar_unpack!(msg, _, _, _, _, {
                if client_id.is_none() && !OP_IN_PROGRESS.load(Ordering::Relaxed) {
                    SUSPEND_PENDING.store(true, Ordering::Relaxed);
                    xous::return_scalar(msg.sender, 1).expect("couldn't ack AcquireSuspendLock");
                } else {
                    xous::return_scalar(msg.sender, 0).expect("couldn't ack AcquireSuspendLock");
                }
            }),
            Some(Opcode::ReleaseSuspendLock) => msg_blocking_scalar_unpack!(msg, _, _, _, _, {
                SUSPEND_PENDING.store(false, Ordering::Relaxed);
                xous::return_scalar(msg.sender, 1).expect("couldn't ack ReleaseSuspendLock");
            }),
            Some(Opcode::WriteRegion) => {
                let mut buffer = unsafe { Buffer::from_memory_message_mut(msg.body.memory_message_mut().unwrap()) };
                let mut wr = buffer.to_original::<WriteRegion, _>().unwrap();
                let mut authorized = true;
                if let Some(st) = soc_token {
                    if staging_write_protect && ((wr.start >= xous::SOC_REGION_LOC) && (wr.start < xous::LOADER_LOC)) ||
                    !staging_write_protect && ((wr.start >= xous::SOC_REGION_LOC) && (wr.start < xous::SOC_STAGING_GW_LOC)) {
                        // if only the holder of the ID that matches the SoC token can write to the SOC flash area
                        // other areas are not as strictly controlled because signature checks ostensibly should catch
                        // attempts to modify them. However, access to the gateware definition would allow one to rewrite
                        // the boot ROM, which would then change the trust root. Therefore, we check this region specifically.
                        if st != wr.id {
                            wr.result = Some(SpinorError::AccessDenied);
                            authorized = false;
                        }
                    }
                } else {
                    // the soc token MUST be initialized early on, if not, something bad has happened.
                    wr.result = Some(SpinorError::AccessDenied);
                    authorized = false;
                }
                if authorized {
                    match client_id {
                        Some(id) => {
                            if wr.id == id {
                                wr.result = Some(spinor.write_region(&mut wr)); // note: this must reject out-of-bound length requests for security reasons
                            } else {
                                wr.result = Some(SpinorError::IdMismatch);
                            }
                        },
                        _ => {
                            wr.result = Some(SpinorError::NoId);
                        }
                    }
                }
                buffer.replace(wr).expect("couldn't return response code to WriteRegion");
            },
            Some(Opcode::EccError) => msg_scalar_unpack!(msg, address, _overflow, _, _, {
                // just some stand-in code -- should probably do something more clever, e.g. a rolling log
                // plus some error handling callback. But this is in the distant future once we have enough
                // of a system to eventually create such errors...
                log::error!("ECC error reported at 0x{:x}", address);
                let mut saved = false;
                for item in ecc_errors.iter_mut() {
                    if item.is_none() {
                        *item = Some(address as u32);
                        saved = true;
                        break;
                    }
                }
                if !saved {
                    log::error!("ran out of slots to record ECC errors");
                }
            }),
            None => {
                log::error!("couldn't convert opcode");
                break
            }
        }
    }
    // clean up our program
    log::trace!("main loop exit, destroying servers");
    let quitconn = xous::connect(susres_mgr_sid).unwrap();
    xous::send_message(quitconn, xous::Message::new_scalar(api::SusResOps::Quit.to_usize().unwrap(), 0, 0, 0, 0)).unwrap();
    unsafe{xous::disconnect(quitconn).unwrap();}

    xns.unregister_server(spinor_sid).unwrap();
    xous::destroy_server(spinor_sid).unwrap();
    log::trace!("quitting");
    xous::terminate_process(0)
}
