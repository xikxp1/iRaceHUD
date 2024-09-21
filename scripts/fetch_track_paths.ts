#!/usr/bin/env node

import IracingAPI from 'iracing-api';
import axios from 'axios';
import { JSDOM } from 'jsdom';
import * as fs from 'fs'

const trackPathFile = "./static/track_paths/track_paths.json";

async function fetchAndExtractSvgPath(svgUrl: string): Promise<string | null> {
    try {
        const response = await axios.get(svgUrl, { responseType: 'document' })
        const dom = new JSDOM(response.data)
        const svgElement = dom.window.document.querySelector('svg path')

        if (svgElement) {
            return svgElement.getAttribute('d')
        } else {
            console.error(`No <path> element found in SVG at ${svgUrl}`)
            return null
        }
    } catch (error) {
        console.error(`Error fetching SVG from ${svgUrl}: ${error}`)
        return null
    }
}

async function featchTrackPaths() {
    try {
        const iracing = new IracingAPI();
        if (!process.env.IRACING_LOGIN || !process.env.IRACING_PWD) {
            throw new Error("Missing iRacing login credentials");
        }
        await iracing.login(process.env.IRACING_LOGIN, process.env.IRACING_PWD);
        console.log("Successfully logged in");

        const trackAssets = await iracing.track.getTrackAssets();
        console.log("Successfully fetched track assets");

        const trackPaths: { [key: string]: string | null } = {}

        for (const trackId in trackAssets) {
            console.log(`Fetching track path for trackId: ${trackId}`);
            const track = trackAssets[trackId];
            const svgPath = await fetchAndExtractSvgPath(`${track.trackMap}${track.trackMapLayers.active}`);
            trackPaths[track.trackId] = svgPath;
        }

        console.log("Writing track paths to file ${trackPathFile} ...");
        fs.writeFileSync(trackPathFile, JSON.stringify(trackPaths, null, 2));
        console.log(`Successfully wrote track paths to ${trackPathFile}`);
    } catch (error) {
        console.error(`Error fetching track paths: ${error}`);
    }
}

featchTrackPaths();
