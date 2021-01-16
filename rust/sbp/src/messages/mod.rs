// Copyright (C) 2015-2018 Swift Navigation Inc.
// Contact: https://support.swiftnav.com
//
// This source is subject to the license found in the file 'LICENSE' which must
// be be distributed together with this source. All other rights reserved.
//
// THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
// EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
pub mod acquisition;
pub mod bootload;
pub mod ext_events;
pub mod file_io;
pub mod flash;
pub mod gnss;
pub mod imu;
pub mod linux;
pub mod logging;
pub mod mag;
pub mod navigation;
pub mod ndb;
pub mod observation;
pub mod orientation;
pub mod piksi;
pub mod sbas;
pub mod settings;
pub mod solution_meta;
pub mod ssr;
pub mod system;
pub mod tracking;
pub mod unknown;
pub mod user;
pub mod vehicle;
use self::acquisition::MsgAcqResult;
use self::acquisition::MsgAcqResultDepA;
use self::acquisition::MsgAcqResultDepB;
use self::acquisition::MsgAcqResultDepC;
use self::acquisition::MsgAcqSvProfile;
use self::acquisition::MsgAcqSvProfileDep;
use self::bootload::MsgBootloaderHandshakeDepA;
use self::bootload::MsgBootloaderHandshakeReq;
use self::bootload::MsgBootloaderHandshakeResp;
use self::bootload::MsgBootloaderJumpToApp;
use self::bootload::MsgNapDeviceDnaReq;
use self::bootload::MsgNapDeviceDnaResp;
use self::ext_events::MsgExtEvent;
use self::file_io::MsgFileioConfigReq;
use self::file_io::MsgFileioConfigResp;
use self::file_io::MsgFileioReadDirReq;
use self::file_io::MsgFileioReadDirResp;
use self::file_io::MsgFileioReadReq;
use self::file_io::MsgFileioReadResp;
use self::file_io::MsgFileioRemove;
use self::file_io::MsgFileioWriteReq;
use self::file_io::MsgFileioWriteResp;
use self::flash::MsgFlashDone;
use self::flash::MsgFlashErase;
use self::flash::MsgFlashProgram;
use self::flash::MsgFlashReadReq;
use self::flash::MsgFlashReadResp;
use self::flash::MsgM25FlashWriteStatus;
use self::flash::MsgStmFlashLockSector;
use self::flash::MsgStmFlashUnlockSector;
use self::flash::MsgStmUniqueIdReq;
use self::flash::MsgStmUniqueIdResp;
use self::imu::MsgImuAux;
use self::imu::MsgImuRaw;
use self::linux::MsgLinuxCpuState;
use self::linux::MsgLinuxMemState;
use self::linux::MsgLinuxProcessFdCount;
use self::linux::MsgLinuxProcessFdSummary;
use self::linux::MsgLinuxProcessSocketCounts;
use self::linux::MsgLinuxProcessSocketQueues;
use self::linux::MsgLinuxSocketUsage;
use self::linux::MsgLinuxSysState;
use self::logging::MsgFwd;
use self::logging::MsgLog;
use self::logging::MsgPrintDep;
use self::mag::MsgMagRaw;
use self::navigation::MsgAgeCorrections;
use self::navigation::MsgBaselineECEF;
use self::navigation::MsgBaselineECEFDepA;
use self::navigation::MsgBaselineHeadingDepA;
use self::navigation::MsgBaselineNED;
use self::navigation::MsgBaselineNEDDepA;
use self::navigation::MsgDops;
use self::navigation::MsgDopsDepA;
use self::navigation::MsgGPSTime;
use self::navigation::MsgGPSTimeDepA;
use self::navigation::MsgGPSTimeGnss;
use self::navigation::MsgPosECEF;
use self::navigation::MsgPosECEFCov;
use self::navigation::MsgPosECEFCovGnss;
use self::navigation::MsgPosECEFDepA;
use self::navigation::MsgPosECEFGnss;
use self::navigation::MsgPosLLH;
use self::navigation::MsgPosLLHCov;
use self::navigation::MsgPosLLHCovGnss;
use self::navigation::MsgPosLLHDepA;
use self::navigation::MsgPosLLHGnss;
use self::navigation::MsgProtectionLevel;
use self::navigation::MsgUtcTime;
use self::navigation::MsgUtcTimeGnss;
use self::navigation::MsgVelBody;
use self::navigation::MsgVelECEF;
use self::navigation::MsgVelECEFCov;
use self::navigation::MsgVelECEFCovGnss;
use self::navigation::MsgVelECEFDepA;
use self::navigation::MsgVelECEFGnss;
use self::navigation::MsgVelNED;
use self::navigation::MsgVelNEDCov;
use self::navigation::MsgVelNEDCovGnss;
use self::navigation::MsgVelNEDDepA;
use self::navigation::MsgVelNEDGnss;
use self::ndb::MsgNdbEvent;
use self::observation::MsgAlmanacGPS;
use self::observation::MsgAlmanacGPSDep;
use self::observation::MsgAlmanacGlo;
use self::observation::MsgAlmanacGloDep;
use self::observation::MsgBasePosECEF;
use self::observation::MsgBasePosLLH;
use self::observation::MsgEphemerisBds;
use self::observation::MsgEphemerisDepA;
use self::observation::MsgEphemerisDepB;
use self::observation::MsgEphemerisDepC;
use self::observation::MsgEphemerisDepD;
use self::observation::MsgEphemerisGPS;
use self::observation::MsgEphemerisGPSDepE;
use self::observation::MsgEphemerisGPSDepF;
use self::observation::MsgEphemerisGal;
use self::observation::MsgEphemerisGalDepA;
use self::observation::MsgEphemerisGlo;
use self::observation::MsgEphemerisGloDepA;
use self::observation::MsgEphemerisGloDepB;
use self::observation::MsgEphemerisGloDepC;
use self::observation::MsgEphemerisGloDepD;
use self::observation::MsgEphemerisQzss;
use self::observation::MsgEphemerisSbas;
use self::observation::MsgEphemerisSbasDepA;
use self::observation::MsgEphemerisSbasDepB;
use self::observation::MsgGloBiases;
use self::observation::MsgGnssCapb;
use self::observation::MsgGroupDelay;
use self::observation::MsgGroupDelayDepA;
use self::observation::MsgGroupDelayDepB;
use self::observation::MsgIono;
use self::observation::MsgObs;
use self::observation::MsgObsDepA;
use self::observation::MsgObsDepB;
use self::observation::MsgObsDepC;
use self::observation::MsgOsr;
use self::observation::MsgSvAzEl;
use self::observation::MsgSvConfigurationGPSDep;
use self::orientation::MsgAngularRate;
use self::orientation::MsgBaselineHeading;
use self::orientation::MsgOrientEuler;
use self::orientation::MsgOrientQuat;
use self::piksi::MsgAlmanac;
use self::piksi::MsgCellModemStatus;
use self::piksi::MsgCommandOutput;
use self::piksi::MsgCommandReq;
use self::piksi::MsgCommandResp;
use self::piksi::MsgCwResults;
use self::piksi::MsgCwStart;
use self::piksi::MsgDeviceMonitor;
use self::piksi::MsgFrontEndGain;
use self::piksi::MsgIarState;
use self::piksi::MsgInitBaseDep;
use self::piksi::MsgMaskSatellite;
use self::piksi::MsgMaskSatelliteDep;
use self::piksi::MsgNetworkBandwidthUsage;
use self::piksi::MsgNetworkStateReq;
use self::piksi::MsgNetworkStateResp;
use self::piksi::MsgReset;
use self::piksi::MsgResetDep;
use self::piksi::MsgResetFilters;
use self::piksi::MsgSetTime;
use self::piksi::MsgSpecan;
use self::piksi::MsgSpecanDep;
use self::piksi::MsgThreadState;
use self::piksi::MsgUartState;
use self::piksi::MsgUartStateDepa;
use self::sbas::MsgSbasRaw;
use self::settings::MsgSettingsReadByIndexDone;
use self::settings::MsgSettingsReadByIndexReq;
use self::settings::MsgSettingsReadByIndexResp;
use self::settings::MsgSettingsReadReq;
use self::settings::MsgSettingsReadResp;
use self::settings::MsgSettingsRegister;
use self::settings::MsgSettingsRegisterResp;
use self::settings::MsgSettingsSave;
use self::settings::MsgSettingsWrite;
use self::settings::MsgSettingsWriteResp;
use self::solution_meta::MsgSolnMeta;
use self::solution_meta::MsgSolnMetaDepA;
use self::ssr::MsgSsrCodeBiases;
use self::ssr::MsgSsrGridDefinitionDepA;
use self::ssr::MsgSsrGriddedCorrection;
use self::ssr::MsgSsrGriddedCorrectionDepA;
use self::ssr::MsgSsrGriddedCorrectionNoStdDepA;
use self::ssr::MsgSsrOrbitClock;
use self::ssr::MsgSsrOrbitClockDepA;
use self::ssr::MsgSsrPhaseBiases;
use self::ssr::MsgSsrStecCorrection;
use self::ssr::MsgSsrStecCorrectionDepA;
use self::ssr::MsgSsrTileDefinition;
use self::system::MsgCsacTelemetry;
use self::system::MsgCsacTelemetryLabels;
use self::system::MsgDgnssStatus;
use self::system::MsgGnssTimeOffset;
use self::system::MsgGroupMeta;
use self::system::MsgHeartbeat;
use self::system::MsgInsStatus;
use self::system::MsgInsUpdates;
use self::system::MsgStartup;
use self::tracking::MsgMeasurementState;
use self::tracking::MsgTrackingIq;
use self::tracking::MsgTrackingIqDepA;
use self::tracking::MsgTrackingIqDepB;
use self::tracking::MsgTrackingState;
use self::tracking::MsgTrackingStateDepA;
use self::tracking::MsgTrackingStateDepB;
use self::tracking::MsgTrackingStateDetailedDep;
use self::tracking::MsgTrackingStateDetailedDepA;
use self::unknown::Unknown;
use self::user::MsgUserData;
use self::vehicle::MsgOdometry;
use self::vehicle::MsgWheeltick;

use crate::framer::FramerError;
use crate::parser::SbpParse;
use crate::serialize::SbpSerialize;
#[cfg(feature = "sbp_serde")]
use serde::{Deserialize, Serialize};

pub trait SBPMessage: SbpSerialize {
    fn get_message_type(&self) -> u16;
    fn get_sender_id(&self) -> Option<u16>;
    fn set_sender_id(&mut self, new_id: u16);
    fn to_frame(&self) -> std::result::Result<Vec<u8>, FramerError>;
}

#[cfg_attr(feature = "sbp_serde", derive(Serialize, Deserialize))]
#[derive(Debug, Clone)]
pub enum SBP {
    MsgPrintDep(MsgPrintDep),
    MsgTrackingStateDetailedDep(MsgTrackingStateDetailedDep),
    MsgTrackingStateDepB(MsgTrackingStateDepB),
    MsgAcqResultDepB(MsgAcqResultDepB),
    MsgAcqResultDepA(MsgAcqResultDepA),
    MsgTrackingStateDepA(MsgTrackingStateDepA),
    MsgThreadState(MsgThreadState),
    MsgUartStateDepa(MsgUartStateDepa),
    MsgIarState(MsgIarState),
    MsgEphemerisDepA(MsgEphemerisDepA),
    MsgMaskSatelliteDep(MsgMaskSatelliteDep),
    MsgTrackingIqDepA(MsgTrackingIqDepA),
    MsgUartState(MsgUartState),
    MsgAcqSvProfileDep(MsgAcqSvProfileDep),
    MsgAcqResultDepC(MsgAcqResultDepC),
    MsgTrackingStateDetailedDepA(MsgTrackingStateDetailedDepA),
    MsgResetFilters(MsgResetFilters),
    MsgInitBaseDep(MsgInitBaseDep),
    MsgMaskSatellite(MsgMaskSatellite),
    MsgTrackingIqDepB(MsgTrackingIqDepB),
    MsgTrackingIq(MsgTrackingIq),
    MsgAcqSvProfile(MsgAcqSvProfile),
    MsgAcqResult(MsgAcqResult),
    MsgTrackingState(MsgTrackingState),
    MsgObsDepB(MsgObsDepB),
    MsgBasePosLLH(MsgBasePosLLH),
    MsgObsDepA(MsgObsDepA),
    MsgEphemerisDepB(MsgEphemerisDepB),
    MsgEphemerisDepC(MsgEphemerisDepC),
    MsgBasePosECEF(MsgBasePosECEF),
    MsgObsDepC(MsgObsDepC),
    MsgObs(MsgObs),
    MsgSpecanDep(MsgSpecanDep),
    MsgSpecan(MsgSpecan),
    MsgMeasurementState(MsgMeasurementState),
    MsgSetTime(MsgSetTime),
    MsgAlmanac(MsgAlmanac),
    MsgAlmanacGPSDep(MsgAlmanacGPSDep),
    MsgAlmanacGloDep(MsgAlmanacGloDep),
    MsgAlmanacGPS(MsgAlmanacGPS),
    MsgAlmanacGlo(MsgAlmanacGlo),
    MsgGloBiases(MsgGloBiases),
    MsgEphemerisDepD(MsgEphemerisDepD),
    MsgEphemerisGPSDepE(MsgEphemerisGPSDepE),
    MsgEphemerisSbasDepA(MsgEphemerisSbasDepA),
    MsgEphemerisGloDepA(MsgEphemerisGloDepA),
    MsgEphemerisSbasDepB(MsgEphemerisSbasDepB),
    MsgEphemerisGloDepB(MsgEphemerisGloDepB),
    MsgEphemerisGPSDepF(MsgEphemerisGPSDepF),
    MsgEphemerisGloDepC(MsgEphemerisGloDepC),
    MsgEphemerisGloDepD(MsgEphemerisGloDepD),
    MsgEphemerisBds(MsgEphemerisBds),
    MsgEphemerisGPS(MsgEphemerisGPS),
    MsgEphemerisGlo(MsgEphemerisGlo),
    MsgEphemerisSbas(MsgEphemerisSbas),
    MsgEphemerisGal(MsgEphemerisGal),
    MsgEphemerisQzss(MsgEphemerisQzss),
    MsgIono(MsgIono),
    MsgSvConfigurationGPSDep(MsgSvConfigurationGPSDep),
    MsgGroupDelayDepA(MsgGroupDelayDepA),
    MsgGroupDelayDepB(MsgGroupDelayDepB),
    MsgGroupDelay(MsgGroupDelay),
    MsgEphemerisGalDepA(MsgEphemerisGalDepA),
    MsgGnssCapb(MsgGnssCapb),
    MsgSvAzEl(MsgSvAzEl),
    MsgSettingsWrite(MsgSettingsWrite),
    MsgSettingsSave(MsgSettingsSave),
    MsgSettingsReadByIndexReq(MsgSettingsReadByIndexReq),
    MsgFileioReadResp(MsgFileioReadResp),
    MsgSettingsReadReq(MsgSettingsReadReq),
    MsgSettingsReadResp(MsgSettingsReadResp),
    MsgSettingsReadByIndexDone(MsgSettingsReadByIndexDone),
    MsgSettingsReadByIndexResp(MsgSettingsReadByIndexResp),
    MsgFileioReadReq(MsgFileioReadReq),
    MsgFileioReadDirReq(MsgFileioReadDirReq),
    MsgFileioReadDirResp(MsgFileioReadDirResp),
    MsgFileioWriteResp(MsgFileioWriteResp),
    MsgFileioRemove(MsgFileioRemove),
    MsgFileioWriteReq(MsgFileioWriteReq),
    MsgSettingsRegister(MsgSettingsRegister),
    MsgSettingsWriteResp(MsgSettingsWriteResp),
    MsgBootloaderHandshakeDepA(MsgBootloaderHandshakeDepA),
    MsgBootloaderJumpToApp(MsgBootloaderJumpToApp),
    MsgResetDep(MsgResetDep),
    MsgBootloaderHandshakeReq(MsgBootloaderHandshakeReq),
    MsgBootloaderHandshakeResp(MsgBootloaderHandshakeResp),
    MsgDeviceMonitor(MsgDeviceMonitor),
    MsgReset(MsgReset),
    MsgCommandReq(MsgCommandReq),
    MsgCommandResp(MsgCommandResp),
    MsgNetworkStateReq(MsgNetworkStateReq),
    MsgNetworkStateResp(MsgNetworkStateResp),
    MsgCommandOutput(MsgCommandOutput),
    MsgNetworkBandwidthUsage(MsgNetworkBandwidthUsage),
    MsgCellModemStatus(MsgCellModemStatus),
    MsgFrontEndGain(MsgFrontEndGain),
    MsgCwResults(MsgCwResults),
    MsgCwStart(MsgCwStart),
    MsgNapDeviceDnaResp(MsgNapDeviceDnaResp),
    MsgNapDeviceDnaReq(MsgNapDeviceDnaReq),
    MsgFlashDone(MsgFlashDone),
    MsgFlashReadResp(MsgFlashReadResp),
    MsgFlashErase(MsgFlashErase),
    MsgStmFlashLockSector(MsgStmFlashLockSector),
    MsgStmFlashUnlockSector(MsgStmFlashUnlockSector),
    MsgStmUniqueIdResp(MsgStmUniqueIdResp),
    MsgFlashProgram(MsgFlashProgram),
    MsgFlashReadReq(MsgFlashReadReq),
    MsgStmUniqueIdReq(MsgStmUniqueIdReq),
    MsgM25FlashWriteStatus(MsgM25FlashWriteStatus),
    MsgGPSTimeDepA(MsgGPSTimeDepA),
    MsgExtEvent(MsgExtEvent),
    MsgGPSTime(MsgGPSTime),
    MsgUtcTime(MsgUtcTime),
    MsgGPSTimeGnss(MsgGPSTimeGnss),
    MsgUtcTimeGnss(MsgUtcTimeGnss),
    MsgSettingsRegisterResp(MsgSettingsRegisterResp),
    MsgPosECEFDepA(MsgPosECEFDepA),
    MsgPosLLHDepA(MsgPosLLHDepA),
    MsgBaselineECEFDepA(MsgBaselineECEFDepA),
    MsgBaselineNEDDepA(MsgBaselineNEDDepA),
    MsgVelECEFDepA(MsgVelECEFDepA),
    MsgVelNEDDepA(MsgVelNEDDepA),
    MsgDopsDepA(MsgDopsDepA),
    MsgBaselineHeadingDepA(MsgBaselineHeadingDepA),
    MsgDops(MsgDops),
    MsgPosECEF(MsgPosECEF),
    MsgPosLLH(MsgPosLLH),
    MsgBaselineECEF(MsgBaselineECEF),
    MsgBaselineNED(MsgBaselineNED),
    MsgVelECEF(MsgVelECEF),
    MsgVelNED(MsgVelNED),
    MsgBaselineHeading(MsgBaselineHeading),
    MsgAgeCorrections(MsgAgeCorrections),
    MsgPosLLHCov(MsgPosLLHCov),
    MsgVelNEDCov(MsgVelNEDCov),
    MsgVelBody(MsgVelBody),
    MsgPosECEFCov(MsgPosECEFCov),
    MsgVelECEFCov(MsgVelECEFCov),
    MsgProtectionLevel(MsgProtectionLevel),
    MsgOrientQuat(MsgOrientQuat),
    MsgOrientEuler(MsgOrientEuler),
    MsgAngularRate(MsgAngularRate),
    MsgPosECEFGnss(MsgPosECEFGnss),
    MsgPosLLHGnss(MsgPosLLHGnss),
    MsgVelECEFGnss(MsgVelECEFGnss),
    MsgVelNEDGnss(MsgVelNEDGnss),
    MsgPosLLHCovGnss(MsgPosLLHCovGnss),
    MsgVelNEDCovGnss(MsgVelNEDCovGnss),
    MsgPosECEFCovGnss(MsgPosECEFCovGnss),
    MsgVelECEFCovGnss(MsgVelECEFCovGnss),
    MsgNdbEvent(MsgNdbEvent),
    MsgLog(MsgLog),
    MsgFwd(MsgFwd),
    MsgSsrOrbitClockDepA(MsgSsrOrbitClockDepA),
    MsgSsrOrbitClock(MsgSsrOrbitClock),
    MsgSsrCodeBiases(MsgSsrCodeBiases),
    MsgSsrPhaseBiases(MsgSsrPhaseBiases),
    MsgSsrStecCorrectionDepA(MsgSsrStecCorrectionDepA),
    MsgSsrGriddedCorrectionNoStdDepA(MsgSsrGriddedCorrectionNoStdDepA),
    MsgSsrGridDefinitionDepA(MsgSsrGridDefinitionDepA),
    MsgSsrTileDefinition(MsgSsrTileDefinition),
    MsgSsrGriddedCorrectionDepA(MsgSsrGriddedCorrectionDepA),
    MsgSsrStecCorrection(MsgSsrStecCorrection),
    MsgSsrGriddedCorrection(MsgSsrGriddedCorrection),
    MsgOsr(MsgOsr),
    MsgUserData(MsgUserData),
    MsgImuRaw(MsgImuRaw),
    MsgImuAux(MsgImuAux),
    MsgMagRaw(MsgMagRaw),
    MsgOdometry(MsgOdometry),
    MsgWheeltick(MsgWheeltick),
    MsgFileioConfigReq(MsgFileioConfigReq),
    MsgFileioConfigResp(MsgFileioConfigResp),
    MsgSbasRaw(MsgSbasRaw),
    MsgLinuxCpuState(MsgLinuxCpuState),
    MsgLinuxMemState(MsgLinuxMemState),
    MsgLinuxSysState(MsgLinuxSysState),
    MsgLinuxProcessSocketCounts(MsgLinuxProcessSocketCounts),
    MsgLinuxProcessSocketQueues(MsgLinuxProcessSocketQueues),
    MsgLinuxSocketUsage(MsgLinuxSocketUsage),
    MsgLinuxProcessFdCount(MsgLinuxProcessFdCount),
    MsgLinuxProcessFdSummary(MsgLinuxProcessFdSummary),
    MsgStartup(MsgStartup),
    MsgDgnssStatus(MsgDgnssStatus),
    MsgInsStatus(MsgInsStatus),
    MsgCsacTelemetry(MsgCsacTelemetry),
    MsgCsacTelemetryLabels(MsgCsacTelemetryLabels),
    MsgInsUpdates(MsgInsUpdates),
    MsgGnssTimeOffset(MsgGnssTimeOffset),
    MsgGroupMeta(MsgGroupMeta),
    MsgSolnMeta(MsgSolnMeta),
    MsgSolnMetaDepA(MsgSolnMetaDepA),
    MsgHeartbeat(MsgHeartbeat),
    Unknown(Unknown),
}

impl SBP {
    pub fn parse(msg_id: u16, sender_id: u16, payload: &mut &[u8]) -> Result<SBP, crate::Error> {
        let x: Result<SBP, crate::Error> = match msg_id {
            16 => {
                let mut msg: MsgPrintDep = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgPrintDep(msg))
            }
            17 => {
                let mut msg: MsgTrackingStateDetailedDep = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgTrackingStateDetailedDep(msg))
            }
            19 => {
                let mut msg: MsgTrackingStateDepB = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgTrackingStateDepB(msg))
            }
            20 => {
                let mut msg: MsgAcqResultDepB = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgAcqResultDepB(msg))
            }
            21 => {
                let mut msg: MsgAcqResultDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgAcqResultDepA(msg))
            }
            22 => {
                let mut msg: MsgTrackingStateDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgTrackingStateDepA(msg))
            }
            23 => {
                let mut msg: MsgThreadState = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgThreadState(msg))
            }
            24 => {
                let mut msg: MsgUartStateDepa = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgUartStateDepa(msg))
            }
            25 => {
                let mut msg: MsgIarState = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgIarState(msg))
            }
            26 => {
                let mut msg: MsgEphemerisDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisDepA(msg))
            }
            27 => {
                let mut msg: MsgMaskSatelliteDep = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgMaskSatelliteDep(msg))
            }
            28 => {
                let mut msg: MsgTrackingIqDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgTrackingIqDepA(msg))
            }
            29 => {
                let mut msg: MsgUartState = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgUartState(msg))
            }
            30 => {
                let mut msg: MsgAcqSvProfileDep = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgAcqSvProfileDep(msg))
            }
            31 => {
                let mut msg: MsgAcqResultDepC = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgAcqResultDepC(msg))
            }
            33 => {
                let mut msg: MsgTrackingStateDetailedDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgTrackingStateDetailedDepA(msg))
            }
            34 => {
                let mut msg: MsgResetFilters = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgResetFilters(msg))
            }
            35 => {
                let mut msg: MsgInitBaseDep = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgInitBaseDep(msg))
            }
            43 => {
                let mut msg: MsgMaskSatellite = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgMaskSatellite(msg))
            }
            44 => {
                let mut msg: MsgTrackingIqDepB = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgTrackingIqDepB(msg))
            }
            45 => {
                let mut msg: MsgTrackingIq = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgTrackingIq(msg))
            }
            46 => {
                let mut msg: MsgAcqSvProfile = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgAcqSvProfile(msg))
            }
            47 => {
                let mut msg: MsgAcqResult = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgAcqResult(msg))
            }
            65 => {
                let mut msg: MsgTrackingState = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgTrackingState(msg))
            }
            67 => {
                let mut msg: MsgObsDepB = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgObsDepB(msg))
            }
            68 => {
                let mut msg: MsgBasePosLLH = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgBasePosLLH(msg))
            }
            69 => {
                let mut msg: MsgObsDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgObsDepA(msg))
            }
            70 => {
                let mut msg: MsgEphemerisDepB = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisDepB(msg))
            }
            71 => {
                let mut msg: MsgEphemerisDepC = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisDepC(msg))
            }
            72 => {
                let mut msg: MsgBasePosECEF = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgBasePosECEF(msg))
            }
            73 => {
                let mut msg: MsgObsDepC = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgObsDepC(msg))
            }
            74 => {
                let mut msg: MsgObs = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgObs(msg))
            }
            80 => {
                let mut msg: MsgSpecanDep = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSpecanDep(msg))
            }
            81 => {
                let mut msg: MsgSpecan = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSpecan(msg))
            }
            97 => {
                let mut msg: MsgMeasurementState = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgMeasurementState(msg))
            }
            104 => {
                let mut msg: MsgSetTime = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSetTime(msg))
            }
            105 => {
                let mut msg: MsgAlmanac = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgAlmanac(msg))
            }
            112 => {
                let mut msg: MsgAlmanacGPSDep = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgAlmanacGPSDep(msg))
            }
            113 => {
                let mut msg: MsgAlmanacGloDep = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgAlmanacGloDep(msg))
            }
            114 => {
                let mut msg: MsgAlmanacGPS = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgAlmanacGPS(msg))
            }
            115 => {
                let mut msg: MsgAlmanacGlo = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgAlmanacGlo(msg))
            }
            117 => {
                let mut msg: MsgGloBiases = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgGloBiases(msg))
            }
            128 => {
                let mut msg: MsgEphemerisDepD = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisDepD(msg))
            }
            129 => {
                let mut msg: MsgEphemerisGPSDepE = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisGPSDepE(msg))
            }
            130 => {
                let mut msg: MsgEphemerisSbasDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisSbasDepA(msg))
            }
            131 => {
                let mut msg: MsgEphemerisGloDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisGloDepA(msg))
            }
            132 => {
                let mut msg: MsgEphemerisSbasDepB = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisSbasDepB(msg))
            }
            133 => {
                let mut msg: MsgEphemerisGloDepB = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisGloDepB(msg))
            }
            134 => {
                let mut msg: MsgEphemerisGPSDepF = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisGPSDepF(msg))
            }
            135 => {
                let mut msg: MsgEphemerisGloDepC = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisGloDepC(msg))
            }
            136 => {
                let mut msg: MsgEphemerisGloDepD = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisGloDepD(msg))
            }
            137 => {
                let mut msg: MsgEphemerisBds = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisBds(msg))
            }
            138 => {
                let mut msg: MsgEphemerisGPS = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisGPS(msg))
            }
            139 => {
                let mut msg: MsgEphemerisGlo = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisGlo(msg))
            }
            140 => {
                let mut msg: MsgEphemerisSbas = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisSbas(msg))
            }
            141 => {
                let mut msg: MsgEphemerisGal = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisGal(msg))
            }
            142 => {
                let mut msg: MsgEphemerisQzss = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisQzss(msg))
            }
            144 => {
                let mut msg: MsgIono = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgIono(msg))
            }
            145 => {
                let mut msg: MsgSvConfigurationGPSDep = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSvConfigurationGPSDep(msg))
            }
            146 => {
                let mut msg: MsgGroupDelayDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgGroupDelayDepA(msg))
            }
            147 => {
                let mut msg: MsgGroupDelayDepB = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgGroupDelayDepB(msg))
            }
            148 => {
                let mut msg: MsgGroupDelay = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgGroupDelay(msg))
            }
            149 => {
                let mut msg: MsgEphemerisGalDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgEphemerisGalDepA(msg))
            }
            150 => {
                let mut msg: MsgGnssCapb = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgGnssCapb(msg))
            }
            151 => {
                let mut msg: MsgSvAzEl = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSvAzEl(msg))
            }
            160 => {
                let mut msg: MsgSettingsWrite = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSettingsWrite(msg))
            }
            161 => {
                let mut msg: MsgSettingsSave = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSettingsSave(msg))
            }
            162 => {
                let mut msg: MsgSettingsReadByIndexReq = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSettingsReadByIndexReq(msg))
            }
            163 => {
                let mut msg: MsgFileioReadResp = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFileioReadResp(msg))
            }
            164 => {
                let mut msg: MsgSettingsReadReq = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSettingsReadReq(msg))
            }
            165 => {
                let mut msg: MsgSettingsReadResp = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSettingsReadResp(msg))
            }
            166 => {
                let mut msg: MsgSettingsReadByIndexDone = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSettingsReadByIndexDone(msg))
            }
            167 => {
                let mut msg: MsgSettingsReadByIndexResp = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSettingsReadByIndexResp(msg))
            }
            168 => {
                let mut msg: MsgFileioReadReq = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFileioReadReq(msg))
            }
            169 => {
                let mut msg: MsgFileioReadDirReq = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFileioReadDirReq(msg))
            }
            170 => {
                let mut msg: MsgFileioReadDirResp = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFileioReadDirResp(msg))
            }
            171 => {
                let mut msg: MsgFileioWriteResp = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFileioWriteResp(msg))
            }
            172 => {
                let mut msg: MsgFileioRemove = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFileioRemove(msg))
            }
            173 => {
                let mut msg: MsgFileioWriteReq = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFileioWriteReq(msg))
            }
            174 => {
                let mut msg: MsgSettingsRegister = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSettingsRegister(msg))
            }
            175 => {
                let mut msg: MsgSettingsWriteResp = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSettingsWriteResp(msg))
            }
            176 => {
                let mut msg: MsgBootloaderHandshakeDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgBootloaderHandshakeDepA(msg))
            }
            177 => {
                let mut msg: MsgBootloaderJumpToApp = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgBootloaderJumpToApp(msg))
            }
            178 => {
                let mut msg: MsgResetDep = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgResetDep(msg))
            }
            179 => {
                let mut msg: MsgBootloaderHandshakeReq = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgBootloaderHandshakeReq(msg))
            }
            180 => {
                let mut msg: MsgBootloaderHandshakeResp = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgBootloaderHandshakeResp(msg))
            }
            181 => {
                let mut msg: MsgDeviceMonitor = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgDeviceMonitor(msg))
            }
            182 => {
                let mut msg: MsgReset = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgReset(msg))
            }
            184 => {
                let mut msg: MsgCommandReq = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgCommandReq(msg))
            }
            185 => {
                let mut msg: MsgCommandResp = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgCommandResp(msg))
            }
            186 => {
                let mut msg: MsgNetworkStateReq = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgNetworkStateReq(msg))
            }
            187 => {
                let mut msg: MsgNetworkStateResp = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgNetworkStateResp(msg))
            }
            188 => {
                let mut msg: MsgCommandOutput = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgCommandOutput(msg))
            }
            189 => {
                let mut msg: MsgNetworkBandwidthUsage = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgNetworkBandwidthUsage(msg))
            }
            190 => {
                let mut msg: MsgCellModemStatus = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgCellModemStatus(msg))
            }
            191 => {
                let mut msg: MsgFrontEndGain = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFrontEndGain(msg))
            }
            192 => {
                let mut msg: MsgCwResults = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgCwResults(msg))
            }
            193 => {
                let mut msg: MsgCwStart = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgCwStart(msg))
            }
            221 => {
                let mut msg: MsgNapDeviceDnaResp = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgNapDeviceDnaResp(msg))
            }
            222 => {
                let mut msg: MsgNapDeviceDnaReq = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgNapDeviceDnaReq(msg))
            }
            224 => {
                let mut msg: MsgFlashDone = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFlashDone(msg))
            }
            225 => {
                let mut msg: MsgFlashReadResp = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFlashReadResp(msg))
            }
            226 => {
                let mut msg: MsgFlashErase = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFlashErase(msg))
            }
            227 => {
                let mut msg: MsgStmFlashLockSector = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgStmFlashLockSector(msg))
            }
            228 => {
                let mut msg: MsgStmFlashUnlockSector = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgStmFlashUnlockSector(msg))
            }
            229 => {
                let mut msg: MsgStmUniqueIdResp = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgStmUniqueIdResp(msg))
            }
            230 => {
                let mut msg: MsgFlashProgram = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFlashProgram(msg))
            }
            231 => {
                let mut msg: MsgFlashReadReq = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFlashReadReq(msg))
            }
            232 => {
                let mut msg: MsgStmUniqueIdReq = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgStmUniqueIdReq(msg))
            }
            243 => {
                let mut msg: MsgM25FlashWriteStatus = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgM25FlashWriteStatus(msg))
            }
            256 => {
                let mut msg: MsgGPSTimeDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgGPSTimeDepA(msg))
            }
            257 => {
                let mut msg: MsgExtEvent = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgExtEvent(msg))
            }
            258 => {
                let mut msg: MsgGPSTime = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgGPSTime(msg))
            }
            259 => {
                let mut msg: MsgUtcTime = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgUtcTime(msg))
            }
            260 => {
                let mut msg: MsgGPSTimeGnss = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgGPSTimeGnss(msg))
            }
            261 => {
                let mut msg: MsgUtcTimeGnss = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgUtcTimeGnss(msg))
            }
            431 => {
                let mut msg: MsgSettingsRegisterResp = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSettingsRegisterResp(msg))
            }
            512 => {
                let mut msg: MsgPosECEFDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgPosECEFDepA(msg))
            }
            513 => {
                let mut msg: MsgPosLLHDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgPosLLHDepA(msg))
            }
            514 => {
                let mut msg: MsgBaselineECEFDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgBaselineECEFDepA(msg))
            }
            515 => {
                let mut msg: MsgBaselineNEDDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgBaselineNEDDepA(msg))
            }
            516 => {
                let mut msg: MsgVelECEFDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgVelECEFDepA(msg))
            }
            517 => {
                let mut msg: MsgVelNEDDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgVelNEDDepA(msg))
            }
            518 => {
                let mut msg: MsgDopsDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgDopsDepA(msg))
            }
            519 => {
                let mut msg: MsgBaselineHeadingDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgBaselineHeadingDepA(msg))
            }
            520 => {
                let mut msg: MsgDops = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgDops(msg))
            }
            521 => {
                let mut msg: MsgPosECEF = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgPosECEF(msg))
            }
            522 => {
                let mut msg: MsgPosLLH = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgPosLLH(msg))
            }
            523 => {
                let mut msg: MsgBaselineECEF = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgBaselineECEF(msg))
            }
            524 => {
                let mut msg: MsgBaselineNED = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgBaselineNED(msg))
            }
            525 => {
                let mut msg: MsgVelECEF = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgVelECEF(msg))
            }
            526 => {
                let mut msg: MsgVelNED = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgVelNED(msg))
            }
            527 => {
                let mut msg: MsgBaselineHeading = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgBaselineHeading(msg))
            }
            528 => {
                let mut msg: MsgAgeCorrections = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgAgeCorrections(msg))
            }
            529 => {
                let mut msg: MsgPosLLHCov = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgPosLLHCov(msg))
            }
            530 => {
                let mut msg: MsgVelNEDCov = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgVelNEDCov(msg))
            }
            531 => {
                let mut msg: MsgVelBody = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgVelBody(msg))
            }
            532 => {
                let mut msg: MsgPosECEFCov = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgPosECEFCov(msg))
            }
            533 => {
                let mut msg: MsgVelECEFCov = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgVelECEFCov(msg))
            }
            534 => {
                let mut msg: MsgProtectionLevel = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgProtectionLevel(msg))
            }
            544 => {
                let mut msg: MsgOrientQuat = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgOrientQuat(msg))
            }
            545 => {
                let mut msg: MsgOrientEuler = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgOrientEuler(msg))
            }
            546 => {
                let mut msg: MsgAngularRate = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgAngularRate(msg))
            }
            553 => {
                let mut msg: MsgPosECEFGnss = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgPosECEFGnss(msg))
            }
            554 => {
                let mut msg: MsgPosLLHGnss = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgPosLLHGnss(msg))
            }
            557 => {
                let mut msg: MsgVelECEFGnss = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgVelECEFGnss(msg))
            }
            558 => {
                let mut msg: MsgVelNEDGnss = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgVelNEDGnss(msg))
            }
            561 => {
                let mut msg: MsgPosLLHCovGnss = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgPosLLHCovGnss(msg))
            }
            562 => {
                let mut msg: MsgVelNEDCovGnss = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgVelNEDCovGnss(msg))
            }
            564 => {
                let mut msg: MsgPosECEFCovGnss = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgPosECEFCovGnss(msg))
            }
            565 => {
                let mut msg: MsgVelECEFCovGnss = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgVelECEFCovGnss(msg))
            }
            1024 => {
                let mut msg: MsgNdbEvent = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgNdbEvent(msg))
            }
            1025 => {
                let mut msg: MsgLog = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgLog(msg))
            }
            1026 => {
                let mut msg: MsgFwd = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFwd(msg))
            }
            1500 => {
                let mut msg: MsgSsrOrbitClockDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSsrOrbitClockDepA(msg))
            }
            1501 => {
                let mut msg: MsgSsrOrbitClock = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSsrOrbitClock(msg))
            }
            1505 => {
                let mut msg: MsgSsrCodeBiases = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSsrCodeBiases(msg))
            }
            1510 => {
                let mut msg: MsgSsrPhaseBiases = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSsrPhaseBiases(msg))
            }
            1515 => {
                let mut msg: MsgSsrStecCorrectionDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSsrStecCorrectionDepA(msg))
            }
            1520 => {
                let mut msg: MsgSsrGriddedCorrectionNoStdDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSsrGriddedCorrectionNoStdDepA(msg))
            }
            1525 => {
                let mut msg: MsgSsrGridDefinitionDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSsrGridDefinitionDepA(msg))
            }
            1526 => {
                let mut msg: MsgSsrTileDefinition = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSsrTileDefinition(msg))
            }
            1530 => {
                let mut msg: MsgSsrGriddedCorrectionDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSsrGriddedCorrectionDepA(msg))
            }
            1531 => {
                let mut msg: MsgSsrStecCorrection = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSsrStecCorrection(msg))
            }
            1532 => {
                let mut msg: MsgSsrGriddedCorrection = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSsrGriddedCorrection(msg))
            }
            1600 => {
                let mut msg: MsgOsr = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgOsr(msg))
            }
            2048 => {
                let mut msg: MsgUserData = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgUserData(msg))
            }
            2304 => {
                let mut msg: MsgImuRaw = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgImuRaw(msg))
            }
            2305 => {
                let mut msg: MsgImuAux = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgImuAux(msg))
            }
            2306 => {
                let mut msg: MsgMagRaw = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgMagRaw(msg))
            }
            2307 => {
                let mut msg: MsgOdometry = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgOdometry(msg))
            }
            2308 => {
                let mut msg: MsgWheeltick = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgWheeltick(msg))
            }
            4097 => {
                let mut msg: MsgFileioConfigReq = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFileioConfigReq(msg))
            }
            4098 => {
                let mut msg: MsgFileioConfigResp = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgFileioConfigResp(msg))
            }
            30583 => {
                let mut msg: MsgSbasRaw = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSbasRaw(msg))
            }
            32512 => {
                let mut msg: MsgLinuxCpuState = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgLinuxCpuState(msg))
            }
            32513 => {
                let mut msg: MsgLinuxMemState = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgLinuxMemState(msg))
            }
            32514 => {
                let mut msg: MsgLinuxSysState = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgLinuxSysState(msg))
            }
            32515 => {
                let mut msg: MsgLinuxProcessSocketCounts = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgLinuxProcessSocketCounts(msg))
            }
            32516 => {
                let mut msg: MsgLinuxProcessSocketQueues = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgLinuxProcessSocketQueues(msg))
            }
            32517 => {
                let mut msg: MsgLinuxSocketUsage = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgLinuxSocketUsage(msg))
            }
            32518 => {
                let mut msg: MsgLinuxProcessFdCount = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgLinuxProcessFdCount(msg))
            }
            32519 => {
                let mut msg: MsgLinuxProcessFdSummary = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgLinuxProcessFdSummary(msg))
            }
            65280 => {
                let mut msg: MsgStartup = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgStartup(msg))
            }
            65282 => {
                let mut msg: MsgDgnssStatus = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgDgnssStatus(msg))
            }
            65283 => {
                let mut msg: MsgInsStatus = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgInsStatus(msg))
            }
            65284 => {
                let mut msg: MsgCsacTelemetry = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgCsacTelemetry(msg))
            }
            65285 => {
                let mut msg: MsgCsacTelemetryLabels = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgCsacTelemetryLabels(msg))
            }
            65286 => {
                let mut msg: MsgInsUpdates = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgInsUpdates(msg))
            }
            65287 => {
                let mut msg: MsgGnssTimeOffset = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgGnssTimeOffset(msg))
            }
            65290 => {
                let mut msg: MsgGroupMeta = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgGroupMeta(msg))
            }
            65294 => {
                let mut msg: MsgSolnMeta = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSolnMeta(msg))
            }
            65295 => {
                let mut msg: MsgSolnMetaDepA = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgSolnMetaDepA(msg))
            }
            65535 => {
                let mut msg: MsgHeartbeat = payload.parse()?;
                msg.set_sender_id(sender_id);
                Ok(SBP::MsgHeartbeat(msg))
            }
            _ => Ok(SBP::Unknown(Unknown {
                msg_id: msg_id,
                sender_id: sender_id,
                payload: payload.to_vec(),
            })),
        };
        match x {
            Ok(x) => Ok(x),
            Err(_) => Err(crate::Error::ParseError),
        }
    }

    pub fn as_sbp_message<'a>(&'a self) -> &dyn SBPMessage {
        match self {
            SBP::MsgPrintDep(msg) => msg,
            SBP::MsgTrackingStateDetailedDep(msg) => msg,
            SBP::MsgTrackingStateDepB(msg) => msg,
            SBP::MsgAcqResultDepB(msg) => msg,
            SBP::MsgAcqResultDepA(msg) => msg,
            SBP::MsgTrackingStateDepA(msg) => msg,
            SBP::MsgThreadState(msg) => msg,
            SBP::MsgUartStateDepa(msg) => msg,
            SBP::MsgIarState(msg) => msg,
            SBP::MsgEphemerisDepA(msg) => msg,
            SBP::MsgMaskSatelliteDep(msg) => msg,
            SBP::MsgTrackingIqDepA(msg) => msg,
            SBP::MsgUartState(msg) => msg,
            SBP::MsgAcqSvProfileDep(msg) => msg,
            SBP::MsgAcqResultDepC(msg) => msg,
            SBP::MsgTrackingStateDetailedDepA(msg) => msg,
            SBP::MsgResetFilters(msg) => msg,
            SBP::MsgInitBaseDep(msg) => msg,
            SBP::MsgMaskSatellite(msg) => msg,
            SBP::MsgTrackingIqDepB(msg) => msg,
            SBP::MsgTrackingIq(msg) => msg,
            SBP::MsgAcqSvProfile(msg) => msg,
            SBP::MsgAcqResult(msg) => msg,
            SBP::MsgTrackingState(msg) => msg,
            SBP::MsgObsDepB(msg) => msg,
            SBP::MsgBasePosLLH(msg) => msg,
            SBP::MsgObsDepA(msg) => msg,
            SBP::MsgEphemerisDepB(msg) => msg,
            SBP::MsgEphemerisDepC(msg) => msg,
            SBP::MsgBasePosECEF(msg) => msg,
            SBP::MsgObsDepC(msg) => msg,
            SBP::MsgObs(msg) => msg,
            SBP::MsgSpecanDep(msg) => msg,
            SBP::MsgSpecan(msg) => msg,
            SBP::MsgMeasurementState(msg) => msg,
            SBP::MsgSetTime(msg) => msg,
            SBP::MsgAlmanac(msg) => msg,
            SBP::MsgAlmanacGPSDep(msg) => msg,
            SBP::MsgAlmanacGloDep(msg) => msg,
            SBP::MsgAlmanacGPS(msg) => msg,
            SBP::MsgAlmanacGlo(msg) => msg,
            SBP::MsgGloBiases(msg) => msg,
            SBP::MsgEphemerisDepD(msg) => msg,
            SBP::MsgEphemerisGPSDepE(msg) => msg,
            SBP::MsgEphemerisSbasDepA(msg) => msg,
            SBP::MsgEphemerisGloDepA(msg) => msg,
            SBP::MsgEphemerisSbasDepB(msg) => msg,
            SBP::MsgEphemerisGloDepB(msg) => msg,
            SBP::MsgEphemerisGPSDepF(msg) => msg,
            SBP::MsgEphemerisGloDepC(msg) => msg,
            SBP::MsgEphemerisGloDepD(msg) => msg,
            SBP::MsgEphemerisBds(msg) => msg,
            SBP::MsgEphemerisGPS(msg) => msg,
            SBP::MsgEphemerisGlo(msg) => msg,
            SBP::MsgEphemerisSbas(msg) => msg,
            SBP::MsgEphemerisGal(msg) => msg,
            SBP::MsgEphemerisQzss(msg) => msg,
            SBP::MsgIono(msg) => msg,
            SBP::MsgSvConfigurationGPSDep(msg) => msg,
            SBP::MsgGroupDelayDepA(msg) => msg,
            SBP::MsgGroupDelayDepB(msg) => msg,
            SBP::MsgGroupDelay(msg) => msg,
            SBP::MsgEphemerisGalDepA(msg) => msg,
            SBP::MsgGnssCapb(msg) => msg,
            SBP::MsgSvAzEl(msg) => msg,
            SBP::MsgSettingsWrite(msg) => msg,
            SBP::MsgSettingsSave(msg) => msg,
            SBP::MsgSettingsReadByIndexReq(msg) => msg,
            SBP::MsgFileioReadResp(msg) => msg,
            SBP::MsgSettingsReadReq(msg) => msg,
            SBP::MsgSettingsReadResp(msg) => msg,
            SBP::MsgSettingsReadByIndexDone(msg) => msg,
            SBP::MsgSettingsReadByIndexResp(msg) => msg,
            SBP::MsgFileioReadReq(msg) => msg,
            SBP::MsgFileioReadDirReq(msg) => msg,
            SBP::MsgFileioReadDirResp(msg) => msg,
            SBP::MsgFileioWriteResp(msg) => msg,
            SBP::MsgFileioRemove(msg) => msg,
            SBP::MsgFileioWriteReq(msg) => msg,
            SBP::MsgSettingsRegister(msg) => msg,
            SBP::MsgSettingsWriteResp(msg) => msg,
            SBP::MsgBootloaderHandshakeDepA(msg) => msg,
            SBP::MsgBootloaderJumpToApp(msg) => msg,
            SBP::MsgResetDep(msg) => msg,
            SBP::MsgBootloaderHandshakeReq(msg) => msg,
            SBP::MsgBootloaderHandshakeResp(msg) => msg,
            SBP::MsgDeviceMonitor(msg) => msg,
            SBP::MsgReset(msg) => msg,
            SBP::MsgCommandReq(msg) => msg,
            SBP::MsgCommandResp(msg) => msg,
            SBP::MsgNetworkStateReq(msg) => msg,
            SBP::MsgNetworkStateResp(msg) => msg,
            SBP::MsgCommandOutput(msg) => msg,
            SBP::MsgNetworkBandwidthUsage(msg) => msg,
            SBP::MsgCellModemStatus(msg) => msg,
            SBP::MsgFrontEndGain(msg) => msg,
            SBP::MsgCwResults(msg) => msg,
            SBP::MsgCwStart(msg) => msg,
            SBP::MsgNapDeviceDnaResp(msg) => msg,
            SBP::MsgNapDeviceDnaReq(msg) => msg,
            SBP::MsgFlashDone(msg) => msg,
            SBP::MsgFlashReadResp(msg) => msg,
            SBP::MsgFlashErase(msg) => msg,
            SBP::MsgStmFlashLockSector(msg) => msg,
            SBP::MsgStmFlashUnlockSector(msg) => msg,
            SBP::MsgStmUniqueIdResp(msg) => msg,
            SBP::MsgFlashProgram(msg) => msg,
            SBP::MsgFlashReadReq(msg) => msg,
            SBP::MsgStmUniqueIdReq(msg) => msg,
            SBP::MsgM25FlashWriteStatus(msg) => msg,
            SBP::MsgGPSTimeDepA(msg) => msg,
            SBP::MsgExtEvent(msg) => msg,
            SBP::MsgGPSTime(msg) => msg,
            SBP::MsgUtcTime(msg) => msg,
            SBP::MsgGPSTimeGnss(msg) => msg,
            SBP::MsgUtcTimeGnss(msg) => msg,
            SBP::MsgSettingsRegisterResp(msg) => msg,
            SBP::MsgPosECEFDepA(msg) => msg,
            SBP::MsgPosLLHDepA(msg) => msg,
            SBP::MsgBaselineECEFDepA(msg) => msg,
            SBP::MsgBaselineNEDDepA(msg) => msg,
            SBP::MsgVelECEFDepA(msg) => msg,
            SBP::MsgVelNEDDepA(msg) => msg,
            SBP::MsgDopsDepA(msg) => msg,
            SBP::MsgBaselineHeadingDepA(msg) => msg,
            SBP::MsgDops(msg) => msg,
            SBP::MsgPosECEF(msg) => msg,
            SBP::MsgPosLLH(msg) => msg,
            SBP::MsgBaselineECEF(msg) => msg,
            SBP::MsgBaselineNED(msg) => msg,
            SBP::MsgVelECEF(msg) => msg,
            SBP::MsgVelNED(msg) => msg,
            SBP::MsgBaselineHeading(msg) => msg,
            SBP::MsgAgeCorrections(msg) => msg,
            SBP::MsgPosLLHCov(msg) => msg,
            SBP::MsgVelNEDCov(msg) => msg,
            SBP::MsgVelBody(msg) => msg,
            SBP::MsgPosECEFCov(msg) => msg,
            SBP::MsgVelECEFCov(msg) => msg,
            SBP::MsgProtectionLevel(msg) => msg,
            SBP::MsgOrientQuat(msg) => msg,
            SBP::MsgOrientEuler(msg) => msg,
            SBP::MsgAngularRate(msg) => msg,
            SBP::MsgPosECEFGnss(msg) => msg,
            SBP::MsgPosLLHGnss(msg) => msg,
            SBP::MsgVelECEFGnss(msg) => msg,
            SBP::MsgVelNEDGnss(msg) => msg,
            SBP::MsgPosLLHCovGnss(msg) => msg,
            SBP::MsgVelNEDCovGnss(msg) => msg,
            SBP::MsgPosECEFCovGnss(msg) => msg,
            SBP::MsgVelECEFCovGnss(msg) => msg,
            SBP::MsgNdbEvent(msg) => msg,
            SBP::MsgLog(msg) => msg,
            SBP::MsgFwd(msg) => msg,
            SBP::MsgSsrOrbitClockDepA(msg) => msg,
            SBP::MsgSsrOrbitClock(msg) => msg,
            SBP::MsgSsrCodeBiases(msg) => msg,
            SBP::MsgSsrPhaseBiases(msg) => msg,
            SBP::MsgSsrStecCorrectionDepA(msg) => msg,
            SBP::MsgSsrGriddedCorrectionNoStdDepA(msg) => msg,
            SBP::MsgSsrGridDefinitionDepA(msg) => msg,
            SBP::MsgSsrTileDefinition(msg) => msg,
            SBP::MsgSsrGriddedCorrectionDepA(msg) => msg,
            SBP::MsgSsrStecCorrection(msg) => msg,
            SBP::MsgSsrGriddedCorrection(msg) => msg,
            SBP::MsgOsr(msg) => msg,
            SBP::MsgUserData(msg) => msg,
            SBP::MsgImuRaw(msg) => msg,
            SBP::MsgImuAux(msg) => msg,
            SBP::MsgMagRaw(msg) => msg,
            SBP::MsgOdometry(msg) => msg,
            SBP::MsgWheeltick(msg) => msg,
            SBP::MsgFileioConfigReq(msg) => msg,
            SBP::MsgFileioConfigResp(msg) => msg,
            SBP::MsgSbasRaw(msg) => msg,
            SBP::MsgLinuxCpuState(msg) => msg,
            SBP::MsgLinuxMemState(msg) => msg,
            SBP::MsgLinuxSysState(msg) => msg,
            SBP::MsgLinuxProcessSocketCounts(msg) => msg,
            SBP::MsgLinuxProcessSocketQueues(msg) => msg,
            SBP::MsgLinuxSocketUsage(msg) => msg,
            SBP::MsgLinuxProcessFdCount(msg) => msg,
            SBP::MsgLinuxProcessFdSummary(msg) => msg,
            SBP::MsgStartup(msg) => msg,
            SBP::MsgDgnssStatus(msg) => msg,
            SBP::MsgInsStatus(msg) => msg,
            SBP::MsgCsacTelemetry(msg) => msg,
            SBP::MsgCsacTelemetryLabels(msg) => msg,
            SBP::MsgInsUpdates(msg) => msg,
            SBP::MsgGnssTimeOffset(msg) => msg,
            SBP::MsgGroupMeta(msg) => msg,
            SBP::MsgSolnMeta(msg) => msg,
            SBP::MsgSolnMetaDepA(msg) => msg,
            SBP::MsgHeartbeat(msg) => msg,
            SBP::Unknown(msg) => msg,
        }
    }
}
