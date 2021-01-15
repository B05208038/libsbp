/*
 * Copyright (C) 2015-2018 Swift Navigation Inc.
 * Contact: https://support.swiftnav.com
 *
 * This source is subject to the license found in the file 'LICENSE' which must
 * be be distributed together with this source. All other rights reserved.
 *
 * THIS CODE AND INFORMATION IS PROVIDED "AS IS" WITHOUT WARRANTY OF ANY KIND,
 * EITHER EXPRESSED OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND/OR FITNESS FOR A PARTICULAR PURPOSE.
 */

package com.swiftnav.sbp.ssr;

import java.math.BigInteger;

import com.swiftnav.sbp.SBPMessage;
import com.swiftnav.sbp.SBPBinaryException;
import com.swiftnav.sbp.SBPStruct;
import com.swiftnav.sbp.gnss.*;

import org.json.JSONObject;
import org.json.JSONArray;
import com.swiftnav.sbp.SBPStruct;

public class SatelliteAPC extends SBPStruct {
    
    /** GNSS signal identifier (16 bit) */
    public GnssSignal sid;
    
    /** Mean phase center offset, x Y and Z axises. See IGS ANTEX file
format description for coordinate system definition.
 */
    public int[] pco;
    
    /** Elevation dependent phase center variations. First element is 0
degrees elevation from the z axis, subsequent elements represent
elevation variations in 1 degree increments.
 */
    public int[] pcv;
    

    public SatelliteAPC () {}

    @Override
    public SatelliteAPC parse(SBPMessage.Parser parser) throws SBPBinaryException {
        /* Parse fields from binary */
        sid = new GnssSignal().parse(parser);
        pco = parser.getArrayofS16(3);
        pcv = parser.getArrayofS8(20);
        return this;
    }

    @Override
    public void build(SBPMessage.Builder builder) {
        /* Build fields into binary */
        sid.build(builder);
        builder.putArrayofS16(pco, 3);
        builder.putArrayofS8(pcv, 20);
    }

    @Override
    public JSONObject toJSON() {
        JSONObject obj = new JSONObject();
        obj.put("sid", sid.toJSON());
        obj.put("pco", new JSONArray(pco));
        obj.put("pcv", new JSONArray(pcv));
        return obj;
    }
}