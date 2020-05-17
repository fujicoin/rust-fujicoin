// Rust Fujicoin Library
// Written in 2014 by
//   Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

//! Consensus parameters
//!
//! This module provides predefined set of parameters for different chains.
//!

use network::constants::Network;
use util::uint::Uint256;

/// Lowest possible difficulty for Mainnet.
const MAX_BITS_FUJICOIN: Uint256 = Uint256([
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0x00000fffffffffffu64,
]);
/// Lowest possible difficulty for Testnet.
const MAX_BITS_TESTNET: Uint256 = Uint256([
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0x00000fffffffffffu64,
]);
/// Lowest possible difficulty for Regtest.
const MAX_BITS_REGTEST: Uint256 = Uint256([
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0xffffffffffffffffu64,
    0x7fffffffffffffffu64,
]);

#[derive(Debug, Clone)]
/// Parameters that influence chain consensus.
pub struct Params {
    /// Network for which parameters are valid.
    pub network: Network,
    /// Time when BIP16 becomes active.
    pub bip16_time: u32,
    /// Block height at which BIP34 becomes active.
    pub bip34_height: u32,
    /// Block height at which BIP65 becomes active.
    pub bip65_height: u32,
    /// Block height at which BIP66 becomes active.
    pub bip66_height: u32,
    /// Minimum blocks including miner confirmation of the total of 2016 blocks in a retargeting period,
    /// (nPowTargetTimespan / nPowTargetSpacing) which is also used for BIP9 deployments.
    /// Examples: 1916 for 95%, 1512 for testchains.
    pub rule_change_activation_threshold: u32,
    /// Number of blocks with the same set of rules.
    pub miner_confirmation_window: u32,
    /// Proof of work limit value. It contains the lowest possible difficulty.
    pub pow_limit: Uint256,
    /// Expected amount of time to mine one block.
    pub pow_target_spacing: u64,
    /// Difficulty recalculation interval.
    pub pow_target_timespan: u64,
    /// Determines whether minimal difficulty may be used for blocks or not.
    pub allow_min_difficulty_blocks: bool,
    /// Determines whether retargeting is disabled for this network or not.
    pub no_pow_retargeting: bool,
}

impl Params {
    /// Creates parameters set for the given network.
    pub fn new(network: Network) -> Self {
        match network {
            Network::Fujicoin => Params {
                network: Network::Fujicoin,
                bip16_time: 0,
                bip34_height: 0,
                bip65_height: 1965600, // 9e3dfb12b8fce00cfdf96850b6f80f3635c2ecdfd26dbf32278c5046c06aaae8
                bip66_height: 1965600, // 9e3dfb12b8fce00cfdf96850b6f80f3635c2ecdfd26dbf32278c5046c06aaae8
                rule_change_activation_threshold: 1512, // 75% of 2016
                miner_confirmation_window: 2016,
                pow_limit: MAX_BITS_FUJICOIN,
                pow_target_spacing: 60,            // 1 minute.
                pow_target_timespan: 2016 * 60, // 1.4 days.
                allow_min_difficulty_blocks: false,
                no_pow_retargeting: false,
            },
            Network::Testnet => Params {
                network: Network::Testnet,
                bip16_time: 0,
                bip34_height: 1800000,
                bip65_height: 1800000,
                bip66_height: 1800000,
                rule_change_activation_threshold: 1512, // 75%
                miner_confirmation_window: 2016,
                pow_limit: MAX_BITS_TESTNET,
                pow_target_spacing: 60,            // 1 minute.
                pow_target_timespan: 2016 * 60, // 1.4 days.
                allow_min_difficulty_blocks: true,
                no_pow_retargeting: false,
            },
            Network::Regtest => Params {
                network: Network::Regtest,
                bip16_time: 0,
                bip34_height: 1800000,
                bip65_height: 1800000,
                bip66_height: 1800000,
                rule_change_activation_threshold: 108, // 75%
                miner_confirmation_window: 144,
                pow_limit: MAX_BITS_REGTEST,
                pow_target_spacing: 60,            // 1 minute.
                pow_target_timespan: 2016 * 60, // 1.4 days.
                allow_min_difficulty_blocks: true,
                no_pow_retargeting: true,
            },
        }
    }

    /// Calculates the number of blocks between difficulty adjustments.
    pub fn difficulty_adjustment_interval(&self) -> u64 {
        self.pow_target_timespan / self.pow_target_spacing
    }
}
