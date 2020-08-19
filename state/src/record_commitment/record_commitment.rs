// Copyright (C) 2019-2020 Aleo Systems Inc.
// This file is part of the Leo library.

// The Leo library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The Leo library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the Leo library. If not, see <https://www.gnu.org/licenses/>.

use crate::{DPCRecordValues, RecordVerificationError};
use leo_typed::Record as TypedRecord;

use snarkos_dpc::base_dpc::{
    instantiated::{Components, RecordCommitment},
    parameters::SystemParameters,
};
use snarkos_models::algorithms::CommitmentScheme;
use snarkos_utilities::{bytes::ToBytes, to_bytes, FromBytes};

use std::convert::TryFrom;

pub fn verify_record_commitment(
    system_parameters: &SystemParameters<Components>,
    typed_record: &TypedRecord,
) -> Result<DPCRecordValues, RecordVerificationError> {
    // generate a dpc record from the typed record
    let record = DPCRecordValues::try_from(typed_record)?;

    // verify record commitment
    let record_commitment_input = to_bytes![
        record.owner,
        record.is_dummy,
        record.value,
        record.payload,
        record.birth_program_id,
        record.death_program_id,
        record.serial_number_nonce
    ]?;

    let commitment = <RecordCommitment as CommitmentScheme>::Output::read(&record.commitment[..])?;
    let commitment_randomness =
        <RecordCommitment as CommitmentScheme>::Randomness::read(&record.commitment_randomness[..])?;

    let record_commitment = RecordCommitment::commit(
        &system_parameters.record_commitment,
        &record_commitment_input,
        &commitment_randomness,
    )?;

    if record_commitment == commitment {
        Ok(record)
    } else {
        Err(RecordVerificationError::CommitmentsDoNotMatch)
    }
}
