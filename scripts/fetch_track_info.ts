#!/usr/bin/env node

import IracingAPI from 'iracing-api';
import axios from 'axios';
import { JSDOM } from 'jsdom';
import * as fs from 'fs'

const trackPathFile = "./static/track_info_data/track_info.json";

const newStartFinishColor = "oklch(80.1701% 0.156063 81.056275)";
const oldStartFinishColor = "#D82520";

async function fetchSvg(svgUrl: string): Promise<string | null> {
    try {
        const response = await axios.get(svgUrl, { responseType: 'document' })
        const dom = new JSDOM(response.data)
        const svgElement = dom.window.document.querySelector('svg')

        if (svgElement) {
            return svgElement.outerHTML
        } else {
            console.error(`No <svg> element found in SVG at ${svgUrl}`)
            return null
        }
    } catch (error) {
        console.error(`Error fetching SVG from ${svgUrl}: ${error}`)
        return null
    }
}

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

async function fetchTrackInfo() {
    try {
        const iracing = new IracingAPI();
        if (!process.env.IRACING_LOGIN || !process.env.IRACING_PWD) {
            throw new Error("Missing iRacing login credentials");
        }
        await iracing.login(process.env.IRACING_LOGIN, process.env.IRACING_PWD);
        console.log("Successfully logged in");

        const tracks = await iracing.track.getTracks();
        if (tracks == undefined || tracks.length == 0) {
            throw new Error("Failed to fetch track list");
        }
        console.log("Successfully fetched track list");
        let trackInfo: Map<number, any> = new Map();
        tracks.forEach((track) => {
            trackInfo.set(track.trackId, {
                trackName: track.trackName,
                configName: track.configName,
            });
        });

        const trackAssets = await iracing.track.getTrackAssets();
        console.log("Successfully fetched track assets");

        const trackPaths: { [key: number]: any } = {};

        for (const trackId in trackAssets) {
            console.log(`Fetching track path for trackId: ${trackId}`);
            const track = trackAssets[trackId];
            let activePath = await fetchAndExtractSvgPath(`${track.trackMap}${track.trackMapLayers.active}`);
            activePath = activePath ? activePath.split(/[zZ]/)[0] : null;
            activePath = activePath ? activePath?.replace(/\n/g, " ") : null;
            activePath = activePath ? activePath?.replace(/\t/g, "") : null;
            activePath += "Z";
            trackPaths[track.trackId] = {
                "id": track.trackId,
                "trackName": trackInfo.get(track.trackId).trackName,
                "configName": trackInfo.get(track.trackId).configName,
                "activePath": activePath,
            };
            let svg = await fetchSvg(`${track.trackMap}${track.trackMapLayers.startFinish}`);
            if (svg) {
                const regex = new RegExp(`${oldStartFinishColor}`, "gi");
                svg = svg.replace(regex, newStartFinishColor);
                fs.writeFileSync(`./static/track_info_data/start_finish/${track.trackId}.svg`, svg);
            } else {
                console.error(`Failed to fetch SVG for trackId: ${track.trackId}`);
            }
        }

        console.log(`Writing track paths to ${trackPathFile} ...`);
        fs.writeFileSync(trackPathFile, JSON.stringify(trackPaths, null, 2));
        console.log(`Successfully wrote track paths to ${trackPathFile}`);
    } catch (error) {
        console.error(`Error fetching track paths: ${error}`);
    }
}

fetchTrackInfo();
