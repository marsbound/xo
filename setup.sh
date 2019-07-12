# Use it on a freshly cloned repository.

setup() {
    echo Running Setup...
    rm -rf .git
    echo Setup done!
}

echo "You're about to remove git in this directory, are you sure?"
select yn in Yes No; do
    case $yn in
        Yes ) setup; break;;
        No ) exit;;
    esac
done
