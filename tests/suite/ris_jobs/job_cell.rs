use ris_jobs::job_cell::JobCell;
use ris_util::testing::duplicate;

#[test]
fn should_be_mutable_and_create_references() {
    unsafe {
        ris_util::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let mut job_cell = unsafe { JobCell::new(0) };

    {
        let ref1 = job_cell.borrow();
        assert_eq!(*ref1, 0);
    }

    let mut mut_ref = job_cell.as_mut();
    assert_eq!(*mut_ref, 0);
    *mut_ref = 42;
    assert_eq!(*mut_ref, 42);

    let ref2 = job_cell.borrow();
    let ref3 = duplicate(&ref2);
    assert_eq!(*ref2, 42);
    assert_eq!(*ref3, 42);
}

#[test]
#[should_panic]
fn should_panic_when_creating_mutable_reference_while_immutable_ones_exist() {
    unsafe {
        ris_util::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let mut job_cell = unsafe { JobCell::new(0) };
    let ref1 = job_cell.borrow();
    let _mut_ref = job_cell.as_mut();
    drop(ref1);
}

#[test]
#[should_panic]
fn should_panic_when_dereferencing_while_owner_was_dropped() {
    unsafe {
        ris_util::throw::SHOW_MESSAGE_BOX_ON_THROW = false;
    }

    let job_cell = unsafe { JobCell::new(0) };
    let ref1 = job_cell.borrow();
    drop(job_cell);
    let _deref1 = *ref1;
}
