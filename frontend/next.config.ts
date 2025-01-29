import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  /* config options here */
  reactStrictMode: true,
};

export default nextConfig;

const removeImports = require('next-remove-imports')();
module.exports = removeImports({});