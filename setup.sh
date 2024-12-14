FILE=".pixi/.activated"
if [ -f $FILE ]; then
    echo "we are active"
else
    echo "installing extendr"
    R -q -e " remotes::install_github('extendr/rextendr', lib='.pixi/envs/default/lib/R/library/') "
    touch $FILE
fi
