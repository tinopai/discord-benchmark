// Get Process ID
const process = require('process');
console.info(`Application PID: ${process.pid}`);

// Eris and the client
const Eris      = require("eris");
const client    = new Eris(process.env.DISCORD_TOKEN);

// Simple ready handler
client.on('ready', () => {
    console.info(`Connected as '${client.user.username}#${client.user.discriminator}' on ${client.guilds.size} guilds`);
});

// Message handler
client.on("messageCreate", (m) => {
    console.log(`'${m.author.username}'@'${m.member.guild.name}': '${m.content}'`);
});

// Connect the bot
client.connect();