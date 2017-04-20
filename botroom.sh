#!/bin/bash

die() {
	echo $1
	exit 1
}

init_bots() {
	[[ ! -d bots ]] && mkdir bots
	for bot in $(get_bots)
	do
		local bot_path="bots/$bot"
		local reload_bot="false"

		if [ -d "$bot_path" ]; then
			(cd "$bot_path/code"
				x="$(git rev-list master)"
				git pull origin master
				if [ ! "$x" == "$(git rev-list master)" ]; then
					reload_bot="true"
					rm -rf ../i* ../bin
				fi
			)
		else
			git clone "https://github.com/ml47-srl/b.3-$bot.git" bots/$bot/code
			reload_bot="true"
		fi

		if [ "$reload_bot" == "true" ]; then
			cp -r botwrapper $bot_path # created bots/linbot/botwrapper
			(cd $bot_path/botwrapper; cargo build)
			mv $bot_path/botwrapper/target/debug/botwrapper $bot_path/bin
			rm -rf $bot_path/botwrapper
		fi
	done
}

# @bot
# $1 = instance
instance_work() {
	(cd i$1
		ls | grep ^r[0-9]*$ | wc -l
	)
}

# @bot
count_instances() {
	ls | grep ^i[0-9]*$ | wc -l
}

# $1 = bot
exec_bot() {
	echo "Executing bot: '$1'"
	(cd bots/$1
		local count=$(count_instances)
		local newest=$(($count - 1))
		local instance=$newest

		if [[ $count == 0 ]] || [[ $(instance_work $newest) == 2 ]]; then
			mkdir i$count
			./bin "new" i$count
			instance=$count
		else
			while true
			do
				if [ $instance == 0 ]; then
					break
				elif [ $((2 * ($(instance_work $instance)+1))) == $(instance_work $(( $instance - 1 ))) ]; then
					break
				fi
			done
		fi
		proofs_path="../../proofs"
		./bin "exec" i$instance $proofs_path
	)
}

get_bots() {
	cat botnames
}

get_bot_with_highest_prio() {
	local bots=$(get_bots)
	local max_bot=${bot[0]}
	for bot in $bots
	do
		if [ $(get_prio $bot) -gt $(get_prio $max_bot) ]; then
			max_bot=$bot
		fi
	done
	echo $max_bot
}

# $1 = bot
get_prio() { # 1/(1+number_of_execs) * 2^niceness?
	echo '1' # TODO
	# $(()) is only int!
}

exec_correct_bot() {
	local bot=$(get_bot_with_highest_prio)
	local prio=$(get_prio $bot)
	if [ ! $prio == 0 ]; then
		exec_bot $bot
	fi
}

init_bots
exec_correct_bot
git pull origin master

# TODO if time is right => commit

exec ./botroom.sh
