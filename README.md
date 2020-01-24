# laptopctl

### Description
Control your laptop's hidden features. **Right now following features are supported**:

* Toggle CPU Turbo Boost:
    * Long option:  `--no_turbo`
    * Short option: `-t`
* Toogle battery Conservation Mode (*for now, supports Lenovo Ideapads only*):
    * Long option: `--conservation_mode`
    * Short option `-c`
* See if a feature is enabled or not: `laptopctl [OPTION] status`

#### Usage examples
`sudo laptopctl --no_turbo enable`

`laptopctl -c disable`

`laptopctl -t status`


## License
Licensed under GNU General Public License v3.0. See [LICENSE](https://github.com/gkeep/laptopctl/blob/master/LICENSE) for more information.
