/*
 * Copyright (C) 2011-2015 Swift Navigation Inc.
 * Contact: Swift Navigation <dev@swift-nav.com>
 *
 * This source is subject to the license found in the file 'LICENSE' which must
 * be be distributed together with this source. All other rights reserved.
 *
 * THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
 * EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
 */

// This file was auto-generated by generate.py. Do not modify by hand!

#include <stdlib.h>
#include <check.h>

#include "check_suites.h"

int main(void)
{
  int number_failed;
  Suite *s = edc_suite();
  SRunner *sr = srunner_create(s);
  srunner_set_xml(sr, "test_results.xml");
  srunner_add_suite(sr, sbp_suite());
  srunner_add_suite(sr, bitfield_macros_suite());

  // auto-generated tests:
  srunner_add_suite(sr, auto_check_sbp_acquisition_1_suite());
  srunner_add_suite(sr, auto_check_sbp_acquisition_2_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_3_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_4_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_5_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_6_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_7_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_8_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_9_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_10_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_11_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_12_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_13_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_14_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_15_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_16_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_17_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_18_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_19_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_20_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_21_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_22_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_23_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_24_suite());
  srunner_add_suite(sr, auto_check_sbp_observation_25_suite());
  srunner_add_suite(sr, auto_check_sbp_observation_26_suite());
  srunner_add_suite(sr, auto_check_sbp_orientation_27_suite());
  srunner_add_suite(sr, auto_check_sbp_orientation_28_suite());
  srunner_add_suite(sr, auto_check_sbp_orientation_29_suite());
  srunner_add_suite(sr, auto_check_sbp_piksi_30_suite());
  srunner_add_suite(sr, auto_check_sbp_piksi_31_suite());
  srunner_add_suite(sr, auto_check_sbp_piksi_32_suite());
  srunner_add_suite(sr, auto_check_sbp_settings_33_suite());
  srunner_add_suite(sr, auto_check_sbp_settings_34_suite());
  srunner_add_suite(sr, auto_check_sbp_system_35_suite());
  srunner_add_suite(sr, auto_check_sbp_system_36_suite());
  srunner_add_suite(sr, auto_check_sbp_system_37_suite());
  srunner_add_suite(sr, auto_check_sbp_system_38_suite());
  srunner_add_suite(sr, auto_check_sbp_acquisition_39_suite());
  srunner_add_suite(sr, auto_check_sbp_bootload_40_suite());
  srunner_add_suite(sr, auto_check_sbp_ext_events_41_suite());
  srunner_add_suite(sr, auto_check_sbp_logging_42_suite());
  srunner_add_suite(sr, auto_check_sbp_logging_43_suite());
  srunner_add_suite(sr, auto_check_sbp_navigation_44_suite());
  srunner_add_suite(sr, auto_check_sbp_observation_45_suite());
  srunner_add_suite(sr, auto_check_sbp_piksi_46_suite());
  srunner_add_suite(sr, auto_check_sbp_system_47_suite());
  srunner_add_suite(sr, auto_check_sbp_tracking_48_suite());
  srunner_add_suite(sr, auto_check_sbp_tracking_49_suite());
  srunner_add_suite(sr, auto_check_sbp_tracking_50_suite());
  srunner_add_suite(sr, auto_check_sbp_vehicle_51_suite());

  srunner_set_fork_status(sr, CK_NOFORK);
  srunner_run_all(sr, CK_NORMAL);
  number_failed = srunner_ntests_failed(sr);
  srunner_free(sr);
  return (number_failed == 0) ? EXIT_SUCCESS : EXIT_FAILURE;
}
