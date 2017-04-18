#!/bin/bash

die() {
	echo $1
	exit 1
}

[[ ! -d botpairs ]] && mkdir botpairs

# eg. botpair = "linbot-23d4c"

init_botcache() {
	[[ ! -d botcache ]] && mkdir botcache
	for bot in $(get_bots)
	do
		if [ -d "botcache/$bot" ]; then
			(cd "botcache/$bot"; git pull origin master)
		else
			git clone "https://github.com/ml47-srl/b.3-$bot.git" botcache/$bot
		fi
	done
}

# $1 = bot
get_revs() {
	(cd "botcache/$1"; git rev-list master)
}

create_botpair() {
	botpair="$1"
	bot=$(bot_from_botpair $botpair)
	rev=$(rev_from_botpair $botpair)
	local botpair_path="botpairs/$botpair"
	mkdir $botpair_path
	cp -r botwrapper $botpair_path # created botpairs/linbot-23d4c/botwrapper
	cp -r "botcache/$bot" $botpair_path/bot # created botpairs/linbot-23d4c/bot
	(cd $botpair_path/bot; git checkout -q "$rev")
	(cd $botpair_path/botwrapper; cargo build)
	mv $botpair_path/botwrapper/target/debug/botwrapper $botpair_path/bin
	(cd $botpair_path; ls | grep -v ^bin$ | xargs rm -rf)
}

create_missing_botpairs() {
	for bot in $(get_bots)
	do
		for rev in $(get_revs $bot)
		do
			local botpair="$bot-$rev"
			local botpair_path="botpairs/$botpair"
			[[ -f $botpair_path ]] && die 'Here is a file? -- snh'
			if [ ! -d $botpair_path ]; then
				create_botpair $botpair
			fi
		done
	done
}

bot_from_botpair() {
	echo ${1%-*}
}

rev_from_botpair() {
	echo ${1#*-}
}

# @botpair
# $1 = instance
instance_work() {
	(cd i$1
		ls | grep ^r[0-9]*$ | wc -l
	)
}

# @botpair
count_instances() {
	ls | grep ^i[0-9]*$ | wc -l
}

# $1 = botpair
exec_botpair() {
	echo "Executing botpair: '$1'"
	(cd botpairs/$1
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
	cat bots
}

get_botpairs() {
	(cd botpairs
		ls
	)
}

get_botpair_with_highest_prio() {
	local botpairs=$(get_botpairs)
	local max_botpair=${botpairs[0]}
	for botpair in $botpairs
	do
		if [ $(get_prio $botpair) -gt $(get_prio $max_botpair) ]; then
			max_botpair=$botpair
		fi
	done
	echo $max_botpair
}

# $1 = botpair
get_prio() { # 1/(1+number_of_execs) * 2^niceness * 1/(1+number_of_commits_behind)
	echo '1' # TODO
	# $(()) is only int!
}

exec_correct_botpair() {
	local botpair=$(get_botpair_with_highest_prio)
	local prio=$(get_prio $botpair)
	if [ ! $prio == 0 ]; then
		exec_botpair $botpair
	fi
}

init_botcache
create_missing_botpairs
exec_correct_botpair
git pull origin master

# TODO if time is right => commit

exec ./botroom.sh
