import adapter from '@sveltejs/adapter-cloudflare';

/** @type {import('@sveltejs/kit').Config} */
const config = { 
    kit: { adapter: adapter() },
    optimizeDeps: {
        exclude: ['layercake']
    },
};

export default config;